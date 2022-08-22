#/usr/bin/env bash
echo "Generating about page ..."
cargo-about generate about.hbs | sed "s/&quot;/'/g;s/&lt;/</g;s/&gt;/>/g;s/&#x27;/'/g" > thirdparty/THIRDPARTY;
echo "Building app ..."
echo "Compressing executable ..."
upx --best --lzma target/release/wwhatis
echo "Creating bundle ..."
tar -czf wwhatis-release.tar.gz target/release/wwhatis thirdparty/THIRDPARTY README.md
echo "Done"