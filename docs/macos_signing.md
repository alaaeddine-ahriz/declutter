# MacOS Code Signing & Notarization Guide

When building MacOS applications with Tauri, the resulting `.app` or `.dmg` file must be **signed** and **notarized** by Apple. If it is not, MacOS will flag the application as "damaged" and prevent it from running to protect the user.

## The "Damaged App" Error

> "Declutter" is damaged and can't be opened. You should move it to the Trash.

This error is misleading. The file is rarely actually damaged; it simply lacks the cryptographic signature that MacOS expects from trusted software.

## Prerequisites

To sign your app for distribution, you need:
1.  **Apple Developer Program Enrollment**: This costs $99/year. [Enroll here](https://developer.apple.com/programs/).
2.  **A Mac**: To generate the certificates (though you can perform the actual build/sign process in CI/CD like GitHub Actions).

---

## Step 1: Generate Certificates

1.  Log in to the [Apple Developer Portal](https://developer.apple.com/account/).
2.  Go to **Certificates, Identifiers & Profiles**.
3.  Click the **+** button to create a new certificate.
4.  Choose **Developer ID Application** (under "Software").
5.  Follow the instructions to upload a Certificate Signing Request (CSR) from your Mac's Keychain Access.
6.  Download the generated certificate (`.cer` file).
7.  Double-click it to install it into your Keychain.

## Step 2: Export the Certificate

1.  Open **Keychain Access** on your Mac.
2.  Select "My Certificates" from the sidebar.
3.  Find the **Developer ID Application** certificate you just installed.
4.  Right-click it and select **Export**.
5.  Save it as a `.p12` file (e.g., `decutter_cert.p12`).
6.  **Important**: Set a strong password for this file when prompted. You will need this password later.

## Step 3: Get the Base64 String

For GitHub Actions, you need to store the `.p12` file as a text string.
Run this command in your terminal:

```bash
base64 -i path/to/certificate.p12 | pbcopy
```

This copies the Base64 string to your clipboard.

## Step 4: Configure GitHub Secrets

Go to your GitHub repository -> **Settings** -> **Secrets and variables** -> **Actions**.
Add the following Repository Secrets:

| Secret Name | Value |
| :--- | :--- |
| `APPLE_CERTIFICATE` | The Base64 string you copied in Step 3. |
| `APPLE_CERTIFICATE_PASSWORD` | The password you set when exporting the `.p12` file in Step 2. |
| `APPLE_SIGNING_IDENTITY` | The Common Name of your certificate. It usually looks like `Developer ID Application: Your Name (TEAMID)`. You can find this in Keychain Access. |
| `APPLE_ID` | Your Apple ID email address. |
| `APPLE_PASSWORD` | An **App-Specific Password**, NOT your main Apple ID password. Generate one at [appleid.apple.com](https://appleid.apple.com/). |

## Step 5: Update Workflow (Already Done)

Your `release.yml` uses `tauri-apps/tauri-action`, which automatically detects these environment variables and handles signing/notarization for you. No code changes are typically needed if the secrets are named correctly.

## Local Workaround (For Testing)

If you download an unsigned build to your own Mac and want to run it, you can bypass the security check by removing the "quarantine" attribute.

Run this in your terminal:

```bash
xattr -cr /Path/To/Declutter.app
```

*Note: You cannot ask your users to do this. You must sign the app for public distribution.*
