import { Construct } from "constructs";
import { Queue } from "aws-cdk-lib/aws-sqs";

export class QueueConstruct extends Construct {

  constructor(scope: Construct, id: string) {
    super(scope, id);

    new Queue(scope, 'PutQueue', {
      queueName: 'article-1'
    })
  }

}
