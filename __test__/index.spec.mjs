import test from 'ava'

import  * as _ from '../index.js'

test('capitalize', t => {
  t.is(_.capitalize('hello'), 'Hello')
  t.is(_.capitalize('hello world'), 'Hello world')
  t.is(_.capitalize(''), '')
  t.is(_.capitalize('123'), '123')
  t.is(_.capitalize('HeLlO WoRlD'), 'Hello world')
})

test('title_case', t => {
  t.is(_.titleCase('hello'), 'Hello')
  t.is(_.titleCase('hello-world'), 'Hello World')
  t.is(_.titleCase('hello_world'), 'Hello World')
  t.is(_.titleCase('hello.world'), 'Hello World')
  t.is(_.titleCase('hello world'), 'Hello World')
  t.is(_.titleCase(''), '')
  t.is(_.titleCase('123'), '123')
  t.is(_.titleCase('HeLlO WoRlD'), 'Hello World')
})

test('camel', t => {
  t.is(_.camel('hello'), 'hello')
  t.is(_.camel('hello-world'), 'helloWorld')
  t.is(_.camel('hello_world'), 'helloWorld')
  t.is(_.camel('hello.world'), 'helloWorld')
  t.is(_.camel('hello world'), 'helloWorld')
  t.is(_.camel(''), '')
  t.is(_.camel('123'), '123')
  t.is(_.camel('HeLlO WoRlD'), 'helloWorld')
})