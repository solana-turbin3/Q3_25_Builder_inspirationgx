export type ITurbinePrereq = {
  version: "0.1.0";
  name: "q3_pre_reqs_rs";
  address: "TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM";
  instructions: [
    {
      name: "close";
      accounts: [
        {
          name: "user";
          isMut: true;
          isSigner: false;
        },
        {
          name: "account";
          isMut: true;
          isSigner: false;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: "createCollection";
      accounts: [
        {
          name: "creator";
          isMut: true;
          isSigner: true;
        },
        {
          name: "collection";
          isMut: true;
          isSigner: true;
        },
        {
          name: "authority";
          isMut: false;
          isSigner: false;
        },
        {
          name: "mplCoreProgram";
          isMut: false;
          isSigner: false;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: "initialize";
      accounts: [
        {
          name: "user";
          isMut: true;
          isSigner: true;
        },
        {
          name: "account";
          isMut: true;
          isSigner: false;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "github";
          type: "string";
        }
      ];
    },
    {
      name: "submitRs";
      accounts: [
        {
          name: "user";
          isMut: true;
          isSigner: true;
        },
        {
          name: "account";
          isMut: true;
          isSigner: false;
        },
        {
          name: "mint";
          isMut: true;
          isSigner: true;
        },
        {
          name: "collection";
          isMut: true;
          isSigner: false;
        },
        {
          name: "authority";
          isMut: false;
          isSigner: false;
        },
        {
          name: "mplCoreProgram";
          isMut: false;
          isSigner: false;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: "submitTs";
      accounts: [
        {
          name: "user";
          isMut: true;
          isSigner: true;
        },
        {
          name: "account";
          isMut: true;
          isSigner: false;
        },
        {
          name: "mint";
          isMut: true;
          isSigner: true;
        },
        {
          name: "collection";
          isMut: true;
          isSigner: false;
        },
        {
          name: "authority";
          isMut: false;
          isSigner: false;
        },
        {
          name: "mplCoreProgram";
          isMut: false;
          isSigner: false;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: "update";
      accounts: [
        {
          name: "user";
          isMut: true;
          isSigner: true;
        },
        {
          name: "account";
          isMut: false;
          isSigner: false;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "github";
          type: "string";
        }
      ];
    }
  ];
  accounts: [
    {
      name: "ApplicationAccount";
      type: {
        kind: "struct";
        fields: [
          {
            name: "user";
            type: "publicKey";
          },
          {
            name: "bump";
            type: "u8";
          },
          {
            name: "preReqTs";
            type: "bool";
          },
          {
            name: "preReqRs";
            type: "bool";
          },
          {
            name: "github";
            type: "string";
          }
        ];
      };
    }
  ];
  types: [
    {
      name: "ApplicationAccount";
      type: {
        kind: "struct";
        fields: [
          {
            name: "user";
            type: "publicKey";
          },
          {
            name: "bump";
            type: "u8";
          },
          {
            name: "preReqTs";
            type: "bool";
          },
          {
            name: "preReqRs";
            type: "bool";
          },
          {
            name: "github";
            type: "string";
          }
        ];
      };
    }
  ];
  errors: [];
  metadata: {
    address: "TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM";
  };
};
