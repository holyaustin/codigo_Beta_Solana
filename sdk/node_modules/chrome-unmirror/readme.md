# chrome-unmirror

**Transform mirror objects from a remote Chrome debugger into local values.**

[![npm status](http://img.shields.io/npm/v/chrome-unmirror.svg?style=flat-square)](https://www.npmjs.org/package/chrome-unmirror) [![node](https://img.shields.io/node/v/chrome-unmirror.svg?style=flat-square)](https://www.npmjs.org/package/chrome-unmirror)

## example

```js
const unmirror = require('chrome-unmirror')

const remoteObject
  = { type: 'object'
    , objectId: '{"injectedScriptId":1,"id":2}'
    , subtype: 'regexp'
    , className: 'RegExp'
    , description: '/flamingo/i'
    , preview:
        { type: 'object'
        , description: '/flamingo/i'
        , lossless: true
        , overflow: false
        , properties:
          [ { name: 'source', type: 'string', value: 'flamingo' }
          , { name: 'global', type: 'boolean', value: 'false' }
          , { name: 'ignoreCase', type: 'boolean', value: 'true' }
          , { name: 'multiline', type: 'boolean', value: 'false' }
          , { name: 'lastIndex', type: 'number', value: '0' } ]
        , subtype: 'regexp' } }

// Returns a RegExp instance: /flamingo/i
unmirror(remoteObject)
```

## install

With [npm](https://npmjs.org) do:

```
npm install chrome-unmirror
```

## tests

Unit tests are currently performed as part of the test suite of an external, unpublished module. Apologies.

## license

[MIT](http://opensource.org/licenses/MIT) © [ironSource](http://www.ironsrc.com/).
