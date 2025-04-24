   1 try { . "c:\Users\gentd\AppData\Local\Programs\Microsoft VS Code\resources\app\out\vs\workbench\contrib\terminal\common\scripts\shellIntegration.ps1" } catch {}
   2 cargo build
   3 vcpkg
   4 vcpkg install openssl
   5 $env:VCPKG_ROOT="C:\Users\gentd\OneDrive\Documents\GitHub\vcpkg"
   6 vcpkg install openssl
   7 cargo build
   8 clear
$env:



=
   

<p align="center">
  <a href="https://solana.com">
    <img alt="Solana" src="https://i.imgur.com/IKyzQ6T.png" width="250" />
  </a>
</p>

# Solana Program

Use the Solana Program Crate to write on-chain programs in Rust.  If writing client-side applications, use the [Solana SDK Crate](https://crates.io/crates/solana-sdk) instead.

More information about Solana is available in the [Solana documentation](https://docs.solana.com/).

[Helloworld](https://github.com/solana-labs/example-helloworld) and the [Solana Program Library](https://github.com/solana-labs/solana-program-library) provide examples of how to use this crate.

Still have questions?  Ask us on [Discord](https://discordapp.com/invite/pquxPsq)
