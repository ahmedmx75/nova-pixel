/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/nova_pixel.json`.
 */
export type NovaPixel = {
  "address": "DRD8K7Ywmpy4JqNE473uBTs6jaf5ajrQ32FxoxzbRoGf",
  "metadata": {
    "name": "novaPixel",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Nova Pixel — Blockchain Pixel War Game"
  },
  "instructions": [
    {
      "name": "adminBuyback",
      "docs": [
        "ADMIN: use buyback SOL to buy NVPX from Jupiter DEX."
      ],
      "discriminator": [
        174,
        118,
        124,
        71,
        5,
        101,
        185,
        123
      ],
      "accounts": [
        {
          "name": "admin",
          "signer": true
        },
        {
          "name": "globalState",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "buybackWalletState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  121,
                  98,
                  97,
                  99,
                  107,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "destinationTokenAccount",
          "docs": [
            "Destination for bought-back NVPX (reserve or liquidity wallet ATA)."
          ],
          "writable": true
        },
        {
          "name": "jupiterProgram"
        }
      ],
      "args": [
        {
          "name": "solAmount",
          "type": "u64"
        },
        {
          "name": "jupiterData",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "burnTokens",
      "docs": [
        "ADMIN: one-time burn of the 150,000,000 NVPX burn allocation (Phase 04)."
      ],
      "discriminator": [
        76,
        15,
        51,
        254,
        229,
        215,
        121,
        66
      ],
      "accounts": [
        {
          "name": "admin",
          "signer": true
        },
        {
          "name": "globalState",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "burnWalletState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  114,
                  110,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "burnTokenAccount",
          "writable": true
        },
        {
          "name": "nvpxMint",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "buyItem",
      "docs": [
        "Buy a Shield 3×3, Shield 5×5, or Rocket.  SOL goes to buyback_wallet."
      ],
      "discriminator": [
        80,
        82,
        193,
        201,
        216,
        27,
        70,
        184
      ],
      "accounts": [
        {
          "name": "player",
          "writable": true,
          "signer": true
        },
        {
          "name": "playerAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "player"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "buybackWalletState",
          "docs": [
            "Buyback wallet PDA — receives the SOL."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  121,
                  98,
                  97,
                  99,
                  107,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "itemType",
          "type": "u8"
        },
        {
          "name": "targetX",
          "type": "u16"
        },
        {
          "name": "targetY",
          "type": "u16"
        },
        {
          "name": "defender",
          "type": {
            "option": "pubkey"
          }
        }
      ]
    },
    {
      "name": "buyPackage",
      "docs": [
        "Buy Starter / Advanced / Pro package — routes SOL through Jupiter → NVPX.",
        "Pass Jupiter swap data and all required accounts as remaining_accounts."
      ],
      "discriminator": [
        231,
        83,
        102,
        184,
        109,
        103,
        117,
        2
      ],
      "accounts": [
        {
          "name": "player",
          "writable": true,
          "signer": true
        },
        {
          "name": "playerAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "player"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "solVault",
          "docs": [
            "SOL vault PDA — receives SOL from player then signs the Jupiter CPI."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "airdropWalletState",
          "docs": [
            "Airdrop wallet state — tracks in-pool NVPX balance."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  105,
                  114,
                  100,
                  114,
                  111,
                  112,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "airdropTokenAccount",
          "docs": [
            "NVPX token account owned by airdrop_wallet_state PDA.",
            "Jupiter deposits purchased NVPX here."
          ],
          "writable": true
        },
        {
          "name": "jupiterProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "packageType",
          "type": "u8"
        },
        {
          "name": "jupiterData",
          "type": "bytes"
        },
        {
          "name": "solAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "capturePixel",
      "docs": [
        "Record a pixel capture: attacker takes the 2× reward from defender."
      ],
      "discriminator": [
        20,
        20,
        189,
        142,
        80,
        115,
        177,
        44
      ],
      "accounts": [
        {
          "name": "gameServer",
          "signer": true
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "attackerAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "arg",
                "path": "attackerWallet"
              }
            ]
          }
        },
        {
          "name": "defenderAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "arg",
                "path": "defenderWallet"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "attackerWallet",
          "type": "pubkey"
        },
        {
          "name": "defenderWallet",
          "type": "pubkey"
        },
        {
          "name": "x",
          "type": "u16"
        },
        {
          "name": "y",
          "type": "u16"
        },
        {
          "name": "nvpxReward",
          "type": "u64"
        }
      ]
    },
    {
      "name": "claimAirdrop",
      "docs": [
        "Claim proportional airdrop share after the tournament ends."
      ],
      "discriminator": [
        137,
        50,
        122,
        111,
        89,
        254,
        8,
        20
      ],
      "accounts": [
        {
          "name": "player",
          "writable": true,
          "signer": true
        },
        {
          "name": "playerAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "player"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "airdropWalletState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  105,
                  114,
                  100,
                  114,
                  111,
                  112,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "airdropTokenAccount",
          "docs": [
            "Airdrop pool token account (source of airdrop tokens)."
          ],
          "writable": true
        },
        {
          "name": "playerNvpxAccount",
          "docs": [
            "Player's NVPX token account (destination)."
          ],
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "colorPixel",
      "docs": [
        "Record a pixel coloring event and pay the 2× reward from the airdrop pool."
      ],
      "discriminator": [
        70,
        171,
        13,
        8,
        141,
        208,
        127,
        196
      ],
      "accounts": [
        {
          "name": "gameServer",
          "docs": [
            "Game server — trusted oracle that reports canvas events on-chain."
          ],
          "signer": true
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "playerAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "arg",
                "path": "playerWallet"
              }
            ]
          }
        },
        {
          "name": "airdropWalletState",
          "docs": [
            "Airdrop wallet state — the source of 2× reward funds."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  105,
                  114,
                  100,
                  114,
                  111,
                  112,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "airdropTokenAccount",
          "docs": [
            "NVPX token account owned by airdrop_wallet_state (source of transfer)."
          ],
          "writable": true
        },
        {
          "name": "playerNvpxAccount",
          "docs": [
            "Player's NVPX token account (destination of reward transfer)."
          ],
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "playerWallet",
          "type": "pubkey"
        },
        {
          "name": "x",
          "type": "u16"
        },
        {
          "name": "y",
          "type": "u16"
        },
        {
          "name": "pixelValue",
          "type": "u64"
        },
        {
          "name": "isCorrect",
          "type": "bool"
        }
      ]
    },
    {
      "name": "connectPlayer",
      "docs": [
        "Register a wallet and join a team (0 / 1 / 2)."
      ],
      "discriminator": [
        71,
        25,
        9,
        230,
        32,
        41,
        2,
        2
      ],
      "accounts": [
        {
          "name": "player",
          "writable": true,
          "signer": true
        },
        {
          "name": "globalState",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "playerAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "player"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "team",
          "type": "u8"
        }
      ]
    },
    {
      "name": "emergencyPause",
      "docs": [
        "MULTISIG: cast a pause vote (2-of-3 required to pause)."
      ],
      "discriminator": [
        21,
        143,
        27,
        142,
        200,
        181,
        210,
        255
      ],
      "accounts": [
        {
          "name": "signer",
          "docs": [
            "Must be one of the three registered multisig signers."
          ],
          "signer": true
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "reason",
          "type": "string"
        }
      ]
    },
    {
      "name": "emergencyUnpause",
      "docs": [
        "ADMIN: unpause after security issue is resolved."
      ],
      "discriminator": [
        83,
        249,
        195,
        57,
        206,
        189,
        31,
        85
      ],
      "accounts": [
        {
          "name": "admin",
          "signer": true
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "endTournament",
      "docs": [
        "ADMIN: close the tournament and begin airdrop distribution phase."
      ],
      "discriminator": [
        99,
        158,
        174,
        193,
        250,
        135,
        4,
        156
      ],
      "accounts": [
        {
          "name": "admin",
          "signer": true
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "docs": [
        "Deploy all PDAs and wallet states.  Called once by admin."
      ],
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "nvpxMint"
        },
        {
          "name": "liquidityWalletState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  105,
                  113,
                  117,
                  105,
                  100,
                  105,
                  116,
                  121,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "airdropWalletState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  105,
                  114,
                  100,
                  114,
                  111,
                  112,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "teamWalletState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  97,
                  109,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "developmentWalletState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  100,
                  101,
                  118,
                  101,
                  108,
                  111,
                  112,
                  109,
                  101,
                  110,
                  116,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "burnWalletState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  114,
                  110,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "reserveWalletState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  115,
                  101,
                  114,
                  118,
                  101,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "buybackWalletState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  121,
                  98,
                  97,
                  99,
                  107,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "initializeParams"
            }
          }
        }
      ]
    },
    {
      "name": "revokeAuthorities",
      "docs": [
        "ADMIN: permanently revoke mint + freeze authority — token becomes fully immutable.",
        "Call once after full supply is minted. IRREVERSIBLE."
      ],
      "discriminator": [
        228,
        135,
        11,
        88,
        103,
        125,
        144,
        223
      ],
      "accounts": [
        {
          "name": "admin",
          "docs": [
            "Must be the current mint authority AND freeze authority."
          ],
          "signer": true
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "nvpxMint",
          "docs": [
            "The NVPX mint — admin must be the current mint authority."
          ],
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "rocketResolve",
      "docs": [
        "Game server removes a defender's shield after a Rocket purchase resolves."
      ],
      "discriminator": [
        199,
        8,
        240,
        155,
        31,
        63,
        191,
        213
      ],
      "accounts": [
        {
          "name": "gameServer",
          "signer": true
        },
        {
          "name": "globalState",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "defenderAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "arg",
                "path": "defenderWallet"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "defenderWallet",
          "type": "pubkey"
        },
        {
          "name": "targetX",
          "type": "u16"
        },
        {
          "name": "targetY",
          "type": "u16"
        }
      ]
    },
    {
      "name": "sellIngame",
      "docs": [
        "Sell in-game NVPX during an active tournament.  50% penalty + 2% tax applied."
      ],
      "discriminator": [
        167,
        32,
        51,
        123,
        78,
        150,
        162,
        248
      ],
      "accounts": [
        {
          "name": "player",
          "writable": true,
          "signer": true
        },
        {
          "name": "playerAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "player"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "airdropWalletState",
          "docs": [
            "Airdrop wallet state — receives the 50% penalty."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  105,
                  114,
                  100,
                  114,
                  111,
                  112,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "developmentWalletState",
          "docs": [
            "Development wallet state — receives the 2% tax."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  100,
                  101,
                  118,
                  101,
                  108,
                  111,
                  112,
                  109,
                  101,
                  110,
                  116,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "playerNvpxAccount",
          "docs": [
            "Player's NVPX token account (source — their in-game holdings)."
          ],
          "writable": true
        },
        {
          "name": "airdropTokenAccount",
          "docs": [
            "Airdrop pool token account (destination for penalty)."
          ],
          "writable": true
        },
        {
          "name": "developmentTokenAccount",
          "docs": [
            "Development wallet token account (destination for tax)."
          ],
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "startTournament",
      "docs": [
        "ADMIN: open the tournament."
      ],
      "discriminator": [
        164,
        168,
        208,
        157,
        43,
        10,
        220,
        241
      ],
      "accounts": [
        {
          "name": "admin",
          "signer": true
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "withdrawLocked",
      "docs": [
        "ADMIN: withdraw from a time-unlocked development or reserve wallet."
      ],
      "discriminator": [
        96,
        224,
        88,
        102,
        223,
        189,
        8,
        228
      ],
      "accounts": [
        {
          "name": "admin",
          "signer": true
        },
        {
          "name": "globalState",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "walletState",
          "writable": true
        },
        {
          "name": "sourceTokenAccount",
          "writable": true
        },
        {
          "name": "destinationTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "globalState",
      "discriminator": [
        163,
        46,
        74,
        168,
        216,
        123,
        133,
        98
      ]
    },
    {
      "name": "playerAccount",
      "discriminator": [
        224,
        184,
        224,
        50,
        98,
        72,
        48,
        236
      ]
    },
    {
      "name": "walletState",
      "discriminator": [
        126,
        186,
        0,
        158,
        92,
        223,
        167,
        68
      ]
    }
  ],
  "events": [
    {
      "name": "airdropClaimed",
      "discriminator": [
        125,
        251,
        195,
        183,
        202,
        126,
        89,
        68
      ]
    },
    {
      "name": "buybackExecuted",
      "discriminator": [
        150,
        109,
        157,
        10,
        124,
        24,
        38,
        189
      ]
    },
    {
      "name": "emergencyPaused",
      "discriminator": [
        97,
        135,
        220,
        149,
        143,
        72,
        9,
        27
      ]
    },
    {
      "name": "emergencyUnpaused",
      "discriminator": [
        94,
        218,
        55,
        35,
        64,
        239,
        172,
        36
      ]
    },
    {
      "name": "inGameSell",
      "discriminator": [
        83,
        223,
        220,
        160,
        98,
        139,
        149,
        213
      ]
    },
    {
      "name": "itemPurchased",
      "discriminator": [
        33,
        219,
        12,
        58,
        205,
        48,
        63,
        143
      ]
    },
    {
      "name": "mintAuthorityRevoked",
      "discriminator": [
        2,
        222,
        125,
        58,
        108,
        126,
        254,
        121
      ]
    },
    {
      "name": "packagePurchased",
      "discriminator": [
        157,
        199,
        235,
        75,
        246,
        165,
        168,
        203
      ]
    },
    {
      "name": "pixelCaptured",
      "discriminator": [
        184,
        65,
        15,
        231,
        17,
        222,
        216,
        43
      ]
    },
    {
      "name": "pixelColored",
      "discriminator": [
        252,
        241,
        92,
        70,
        59,
        101,
        241,
        0
      ]
    },
    {
      "name": "playerConnected",
      "discriminator": [
        44,
        170,
        132,
        192,
        118,
        48,
        157,
        86
      ]
    },
    {
      "name": "rocketFired",
      "discriminator": [
        250,
        177,
        207,
        59,
        217,
        88,
        63,
        210
      ]
    },
    {
      "name": "roundEnded",
      "discriminator": [
        70,
        113,
        6,
        162,
        176,
        78,
        201,
        19
      ]
    },
    {
      "name": "shieldActivated",
      "discriminator": [
        155,
        28,
        228,
        125,
        168,
        131,
        255,
        126
      ]
    },
    {
      "name": "tokensBurned",
      "discriminator": [
        230,
        255,
        34,
        113,
        226,
        53,
        227,
        9
      ]
    },
    {
      "name": "tournamentEnded",
      "discriminator": [
        46,
        250,
        165,
        48,
        249,
        96,
        51,
        134
      ]
    },
    {
      "name": "tournamentStarted",
      "discriminator": [
        200,
        157,
        174,
        194,
        174,
        219,
        107,
        44
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "tournamentNotActive",
      "msg": "Tournament is not active"
    },
    {
      "code": 6001,
      "name": "tournamentAlreadyActive",
      "msg": "Tournament is already active"
    },
    {
      "code": 6002,
      "name": "tournamentAlreadyEnded",
      "msg": "Tournament is already ended"
    },
    {
      "code": 6003,
      "name": "tournamentNotEnded",
      "msg": "Tournament has not ended yet"
    },
    {
      "code": 6004,
      "name": "contractPaused",
      "msg": "Contract is paused — emergency only"
    },
    {
      "code": 6005,
      "name": "notPaused",
      "msg": "Contract is not paused"
    },
    {
      "code": 6006,
      "name": "notAdmin",
      "msg": "Not authorized: admin only"
    },
    {
      "code": 6007,
      "name": "notGameServer",
      "msg": "Not authorized: game server only"
    },
    {
      "code": 6008,
      "name": "notMultisigSigner",
      "msg": "Not a registered multisig signer"
    },
    {
      "code": 6009,
      "name": "alreadyVoted",
      "msg": "Signer already voted for pause"
    },
    {
      "code": 6010,
      "name": "insufficientPauseVotes",
      "msg": "Insufficient pause votes (need 2-of-3)"
    },
    {
      "code": 6011,
      "name": "playerNotInitialized",
      "msg": "Player account not initialized — call connect_player first"
    },
    {
      "code": 6012,
      "name": "invalidTeam",
      "msg": "Invalid team: must be 0, 1, or 2"
    },
    {
      "code": 6013,
      "name": "insufficientAttempts",
      "msg": "Insufficient attempts balance"
    },
    {
      "code": 6014,
      "name": "insufficientBalance",
      "msg": "Insufficient in-game NVPX balance"
    },
    {
      "code": 6015,
      "name": "invalidPackageType",
      "msg": "Invalid package type"
    },
    {
      "code": 6016,
      "name": "invalidItemType",
      "msg": "Invalid item type"
    },
    {
      "code": 6017,
      "name": "insufficientSolForItem",
      "msg": "Insufficient SOL sent for this item"
    },
    {
      "code": 6018,
      "name": "invalidPixelCoords",
      "msg": "Pixel coordinates out of canvas bounds"
    },
    {
      "code": 6019,
      "name": "pixelShielded",
      "msg": "Target pixel is protected by a shield"
    },
    {
      "code": 6020,
      "name": "noShieldAtTarget",
      "msg": "No active shield found at target location"
    },
    {
      "code": 6021,
      "name": "tooManyShields",
      "msg": "Player has reached the maximum number of active shields"
    },
    {
      "code": 6022,
      "name": "zeroPixelValue",
      "msg": "Zero pixel value — must be positive"
    },
    {
      "code": 6023,
      "name": "airdropAlreadyClaimed",
      "msg": "Airdrop already claimed"
    },
    {
      "code": 6024,
      "name": "noAirdropAllocation",
      "msg": "No airdrop allocation for this player"
    },
    {
      "code": 6025,
      "name": "airdropPoolInsufficient",
      "msg": "Airdrop pool has insufficient funds"
    },
    {
      "code": 6026,
      "name": "walletLocked",
      "msg": "Wallet is still time-locked"
    },
    {
      "code": 6027,
      "name": "alreadyBurned",
      "msg": "Tokens have already been burned"
    },
    {
      "code": 6028,
      "name": "authorityAlreadyRevoked",
      "msg": "Mint and freeze authorities have already been permanently revoked"
    },
    {
      "code": 6029,
      "name": "mathOverflow",
      "msg": "Arithmetic overflow"
    },
    {
      "code": 6030,
      "name": "mathUnderflow",
      "msg": "Arithmetic underflow"
    },
    {
      "code": 6031,
      "name": "divisionByZero",
      "msg": "Division by zero"
    },
    {
      "code": 6032,
      "name": "invalidSellAmount",
      "msg": "Invalid sell amount — must be positive"
    },
    {
      "code": 6033,
      "name": "jupiterSwapFailed",
      "msg": "Jupiter swap CPI failed"
    },
    {
      "code": 6034,
      "name": "invalidJupiterProgram",
      "msg": "Invalid Jupiter program ID"
    },
    {
      "code": 6035,
      "name": "slippageExceeded",
      "msg": "Slippage tolerance exceeded"
    },
    {
      "code": 6036,
      "name": "zeroTokensReceived",
      "msg": "Jupiter returned zero tokens"
    }
  ],
  "types": [
    {
      "name": "airdropClaimed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "buybackExecuted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "solSpent",
            "type": "u64"
          },
          {
            "name": "nvpxBought",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "emergencyPaused",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reason",
            "type": "string"
          },
          {
            "name": "signer",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "emergencyUnpaused",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "signer",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "globalState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "gameServer",
            "type": "pubkey"
          },
          {
            "name": "multisigSigners",
            "type": {
              "array": [
                "pubkey",
                3
              ]
            }
          },
          {
            "name": "pauseVotes",
            "type": {
              "array": [
                "bool",
                3
              ]
            }
          },
          {
            "name": "tournamentState",
            "type": {
              "defined": {
                "name": "tournamentState"
              }
            }
          },
          {
            "name": "tournamentStartTime",
            "type": "i64"
          },
          {
            "name": "tournamentEndTime",
            "type": "i64"
          },
          {
            "name": "initializedAt",
            "type": "i64"
          },
          {
            "name": "currentRound",
            "type": "u8"
          },
          {
            "name": "airdropFinalized",
            "type": "bool"
          },
          {
            "name": "authoritiesRevoked",
            "type": "bool"
          },
          {
            "name": "nvpxMint",
            "type": "pubkey"
          },
          {
            "name": "totalCorrectPixels",
            "type": "u64"
          },
          {
            "name": "totalNvpxInGame",
            "type": "u64"
          },
          {
            "name": "totalNvpxRewarded",
            "type": "u64"
          },
          {
            "name": "liquidityWallet",
            "type": "pubkey"
          },
          {
            "name": "airdropWallet",
            "type": "pubkey"
          },
          {
            "name": "teamWallet",
            "type": "pubkey"
          },
          {
            "name": "developmentWallet",
            "type": "pubkey"
          },
          {
            "name": "burnWallet",
            "type": "pubkey"
          },
          {
            "name": "reserveWallet",
            "type": "pubkey"
          },
          {
            "name": "buybackWallet",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "inGameSell",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "nvpxSold",
            "type": "u64"
          },
          {
            "name": "penaltyAmount",
            "type": "u64"
          },
          {
            "name": "taxAmount",
            "type": "u64"
          },
          {
            "name": "receivedAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "initializeParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "gameServer",
            "type": "pubkey"
          },
          {
            "name": "multisigSigners",
            "type": {
              "array": [
                "pubkey",
                3
              ]
            }
          },
          {
            "name": "useDevnetTimelocks",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "itemPurchased",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "itemType",
            "type": "u8"
          },
          {
            "name": "solCost",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "mintAuthorityRevoked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "packagePurchased",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "packageType",
            "type": "u8"
          },
          {
            "name": "attempts",
            "type": "u32"
          },
          {
            "name": "solPaid",
            "type": "u64"
          },
          {
            "name": "nvpxBought",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "pixelCaptured",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attackerWallet",
            "type": "pubkey"
          },
          {
            "name": "defenderWallet",
            "type": "pubkey"
          },
          {
            "name": "x",
            "type": "u16"
          },
          {
            "name": "y",
            "type": "u16"
          },
          {
            "name": "nvpxTransferred",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "pixelColored",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "team",
            "type": "u8"
          },
          {
            "name": "x",
            "type": "u16"
          },
          {
            "name": "y",
            "type": "u16"
          },
          {
            "name": "isCorrect",
            "type": "bool"
          },
          {
            "name": "nvpxReward",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "playerAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "walletAddress",
            "type": "pubkey"
          },
          {
            "name": "team",
            "type": "u8"
          },
          {
            "name": "attemptsBalance",
            "type": "u32"
          },
          {
            "name": "inGameNvpxBalance",
            "type": "u64"
          },
          {
            "name": "correctPixelsColored",
            "type": "u64"
          },
          {
            "name": "airdropAllocation",
            "type": "u64"
          },
          {
            "name": "airdropClaimed",
            "type": "bool"
          },
          {
            "name": "joinTimestamp",
            "type": "i64"
          },
          {
            "name": "totalSolSpent",
            "type": "u64"
          },
          {
            "name": "activeShields",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "shield"
                  }
                },
                10
              ]
            }
          },
          {
            "name": "isInitialized",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "playerConnected",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "team",
            "type": "u8"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "rocketFired",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "targetX",
            "type": "u16"
          },
          {
            "name": "targetY",
            "type": "u16"
          },
          {
            "name": "shieldDestroyed",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "roundEnded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "round",
            "type": "u8"
          },
          {
            "name": "timestamp",
            "type": "i64"
          },
          {
            "name": "totalCorrectPixels",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "shield",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "centerX",
            "type": "u16"
          },
          {
            "name": "centerY",
            "type": "u16"
          },
          {
            "name": "size",
            "type": "u8"
          },
          {
            "name": "expiryTime",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "shieldActivated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "team",
            "type": "u8"
          },
          {
            "name": "centerX",
            "type": "u16"
          },
          {
            "name": "centerY",
            "type": "u16"
          },
          {
            "name": "size",
            "type": "u8"
          },
          {
            "name": "expiry",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "tokensBurned",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "tournamentEnded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "timestamp",
            "type": "i64"
          },
          {
            "name": "totalCorrectPixels",
            "type": "u64"
          },
          {
            "name": "totalNvpxInGame",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "tournamentStarted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "tournamentState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "inactive"
          },
          {
            "name": "active"
          },
          {
            "name": "roundEnded"
          },
          {
            "name": "ended"
          },
          {
            "name": "paused"
          },
          {
            "name": "distribution"
          }
        ]
      }
    },
    {
      "name": "walletState",
      "docs": [
        "State PDA for each locked program wallet.",
        "The actual tokens live in the ATA owned by this PDA."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "walletType",
            "type": {
              "defined": {
                "name": "walletType"
              }
            }
          },
          {
            "name": "nvpxBalance",
            "type": "u64"
          },
          {
            "name": "initialAllocation",
            "type": "u64"
          },
          {
            "name": "lockUntil",
            "type": "i64"
          },
          {
            "name": "isBurned",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "walletType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "liquidity"
          },
          {
            "name": "airdrop"
          },
          {
            "name": "team"
          },
          {
            "name": "development"
          },
          {
            "name": "burn"
          },
          {
            "name": "reserve"
          },
          {
            "name": "buyback"
          }
        ]
      }
    }
  ]
};
