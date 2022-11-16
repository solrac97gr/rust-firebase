# Firebase Realtime Database and Rust

It not exist oficial support of the Firebase SDK in Rust, so the only way for work with Firebase as a database is using the REST API of Firebase Realtime Database.

## Solution:

I decided to use a external package `firebase-rs` for communicate with the Firebase Realtime Database REST API and implement a basic crud using this methods.

## What methods I use?:
- Get
- Get All
- Update
- Delete

## Conclusions:

Rust and Firebase is not good combination for now, until the official package is not relase is better not to use Firebase in combination with Rust.