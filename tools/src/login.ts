import got = require('got')
import cheerio = require('cheerio')
import path = require('path')
import fs = require('fs')
import {CookieJar, Cookie} from 'tough-cookie'

const COOKIE_STORE_PATH = path.join(process.env.HOME as string, ".atcoder.json")

function parseCsrfToken(body: string): string {
  const $ = cheerio.load(body)
  return $('input[name="csrf_token"]').val()
}

function saveSession(jar: CookieJar, url: string) {
  const json = JSON.stringify(jar.toJSON())
  fs.writeFileSync(COOKIE_STORE_PATH, json)
}

export function restoreCookieJar(): CookieJar {
  let jar = new CookieJar()
  try {
    const json = fs.readFileSync(COOKIE_STORE_PATH, 'utf8')
    jar = CookieJar.fromJSON(json)
  } catch (e) {}
  return jar
}

export async function login(loginUrl: string) {
  const cookieJar = new CookieJar()

  const res = await got.get(loginUrl, { cookieJar })
  if (res.statusCode != 200) {
    throw new Error(`Error! The server returns ${res.statusCode}`)
  }

  const csrfToken = parseCsrfToken(res.body)
  const formBody = {
    username: process.env.ATCODER_USER,
    password: process.env.ATCODER_PASSWORD,
    csrf_token: csrfToken
  }

  // XXX. Following code dit not work..
  // const postRe = await got.post(..)
  got.post(loginUrl, {
    form: true,
    body: formBody,
    throwHttpErrors: false,
    cookieJar
  })
  .then((postResponse: any) => {
    if (postResponse.statusCode == 302) {
      console.log('Login success!')
      saveSession(cookieJar, loginUrl)
    } else {
      console.log(`Error! Login failed! body=${JSON.stringify(formBody)}`)
    }
  })
}