import { Construct } from "constructs";
import { RustFunction } from 'cargo-lambda-cdk';
import { Architecture, FunctionUrlAuthType } from "aws-cdk-lib/aws-lambda";
import { Effect, PolicyStatement } from "aws-cdk-lib/aws-iam";
import { ITable } from "aws-cdk-lib/aws-dynamodb";

export interface GetLambdaConstructProps {
  table: ITable;
}

export class GetLambdaConstruct extends Construct {
  constructor(scope: Construct, id: string, props: GetLambdaConstructProps) {
    super(scope, id);

    let rustFunction = new RustFunction(scope, 'GetHandler', {
      manifestPath: 'app/get-lambda',
      architecture: Architecture.ARM_64,
      environment: {
        RUST_LOG: "get-lambda=debug",
        TABLE_NAME: props.table.tableName
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
