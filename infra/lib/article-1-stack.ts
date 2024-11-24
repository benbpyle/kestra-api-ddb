import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { TableConstruct } from './constructs/ddb-table';
import { PostLambdaConstruct } from './constructs/post-lambda';
import { QueueConstruct } from './constructs/queue-construct';
import { GetLambdaConstruct } from './constructs/get-lambda';

export class Article1Stack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const tb = new TableConstruct(this, 'TableConstruct');
    const q = new QueueConstruct(this, 'QueueConstruct');
    const post = new PostLambdaConstruct(this, 'PostLambdaConstruct', {
      table: tb.table
    })
    const get = new GetLambdaConstruct(this, 'GetLambdaConstruct', {
      table: tb.table
    })
  }
}
