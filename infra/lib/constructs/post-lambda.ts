import { Construct } from "constructs";
import { RustFunction } from 'cargo-lambda-cdk';
import { Architecture, FunctionUrlAuthType } from "aws-cdk-lib/aws-lambda";
import { Effect, PolicyStatement } from "aws-cdk-lib/aws-iam";
import { ITable } from "aws-cdk-lib/aws-dynamodb";

export interface PostLambdaConstructProps {
  table: ITable;
}

export class PostLambdaConstruct extends Construct {
  constructor(scope: Construct, id: string, props: PostLambdaConstructProps) {
    super(scope, id);

    let rustFunction = new RustFunction(scope, 'PostHandler', {
      manifestPath: 'app/post-lambda',
      architecture: Architecture.ARM_64,
      environment: {
        RUST_LOG: "post-lambda=debug"
      },
      memorySize: 256
    });

    rustFunction.addFunctionUrl({
      authType: FunctionUrlAuthType.NONE
    })

    rustFunction.addToRolePolicy(new PolicyStatement({
      actions: ["dynamodb:*"],
      resources: ["*"],
      effect: Effect.ALLOW
    }))
  }
}
