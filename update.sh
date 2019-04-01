#!/usr/bin/env bash
cp SWM050.svd SWM050-patched.svd
for patch in patches/*
do
	patch \
		--silent \
		--force \
		--reject-file=- \
		SWM050-patched.svd $patch
done

rm -rf src && mkdir src &&
svd2rust -i SWM050-patched.svd &&
form -i lib.rs -o src && rm lib.rs &&
cargo fmt &&
rustfmt build.rs
