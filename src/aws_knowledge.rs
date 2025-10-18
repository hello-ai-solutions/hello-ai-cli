use std::collections::HashMap;

#[allow(dead_code)]
pub struct AWSKnowledgeBase {
    service_patterns: HashMap<String, Vec<String>>,
    best_practices: HashMap<String, Vec<String>>,
    common_patterns: HashMap<String, String>,
}

impl AWSKnowledgeBase {
    pub fn new() -> Self {
        let mut service_patterns = HashMap::new();
        let mut best_practices = HashMap::new();
        let mut common_patterns = HashMap::new();

        // AWS Service Patterns
        service_patterns.insert(
            "s3".to_string(),
            vec![
                "aws s3 cp".to_string(),
                "boto3.client('s3')".to_string(),
                "S3Client".to_string(),
                "PutObjectRequest".to_string(),
            ],
        );

        service_patterns.insert(
            "lambda".to_string(),
            vec![
                "lambda_handler".to_string(),
                "event".to_string(),
                "context".to_string(),
                "boto3.client('lambda')".to_string(),
            ],
        );

        service_patterns.insert(
            "dynamodb".to_string(),
            vec![
                "boto3.resource('dynamodb')".to_string(),
                "Table".to_string(),
                "put_item".to_string(),
                "query".to_string(),
            ],
        );

        // Best Practices
        best_practices.insert(
            "s3".to_string(),
            vec![
                "Use server-side encryption".to_string(),
                "Enable versioning for critical data".to_string(),
                "Implement lifecycle policies".to_string(),
                "Use IAM policies for access control".to_string(),
            ],
        );

        best_practices.insert(
            "lambda".to_string(),
            vec![
                "Keep functions stateless".to_string(),
                "Use environment variables for configuration".to_string(),
                "Implement proper error handling".to_string(),
                "Optimize memory allocation".to_string(),
            ],
        );

        // Common Patterns
        common_patterns.insert(
            "s3_upload".to_string(),
            "s3_client.put_object(Bucket='bucket-name', Key='key', Body=data)".to_string(),
        );
        common_patterns.insert(
            "lambda_response".to_string(),
            "{'statusCode': 200, 'body': json.dumps('Hello World')}".to_string(),
        );
        common_patterns.insert(
            "dynamodb_query".to_string(),
            "table.query(KeyConditionExpression=Key('pk').eq('value'))".to_string(),
        );

        Self {
            service_patterns,
            best_practices,
            common_patterns,
        }
    }

    pub fn get_aws_suggestions(&self, code: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Detect AWS services in code
        for (service, patterns) in &self.service_patterns {
            for pattern in patterns {
                if code.contains(pattern) {
                    if let Some(practices) = self.best_practices.get(service) {
                        suggestions.extend(
                            practices
                                .iter()
                                .map(|p| format!("💡 {}: {}", service.to_uppercase(), p)),
                        );
                    }
                }
            }
        }

        // Suggest common patterns
        if code.contains("s3") && !code.contains("put_object") {
            suggestions.push("💡 S3: Consider using put_object for uploads".to_string());
        }

        if code.contains("lambda") && !code.contains("try:") {
            suggestions.push("💡 Lambda: Add error handling with try/except".to_string());
        }

        suggestions
    }

    pub fn get_cloudformation_template(&self, service: &str) -> Option<String> {
        match service {
            "s3" => Some(
                r#"
Resources:
  MyS3Bucket:
    Type: AWS::S3::Bucket
    Properties:
      BucketName: !Sub "${AWS::StackName}-bucket"
      VersioningConfiguration:
        Status: Enabled
      BucketEncryption:
        ServerSideEncryptionConfiguration:
          - ServerSideEncryptionByDefault:
              SSEAlgorithm: AES256
"#
                .to_string(),
            ),
            "lambda" => Some(
                r#"
Resources:
  MyLambdaFunction:
    Type: AWS::Lambda::Function
    Properties:
      FunctionName: !Sub "${AWS::StackName}-function"
      Runtime: python3.9
      Handler: index.lambda_handler
      Code:
        ZipFile: |
          def lambda_handler(event, context):
              return {'statusCode': 200, 'body': 'Hello World'}
      Role: !GetAtt LambdaExecutionRole.Arn
"#
                .to_string(),
            ),
            _ => None,
        }
    }

    pub fn enhance_with_aws_context(&self, explanation: &str, code: &str) -> String {
        let aws_suggestions = self.get_aws_suggestions(code);

        if aws_suggestions.is_empty() {
            explanation.to_string()
        } else {
            format!(
                "{}\n\n🔧 AWS Recommendations:\n{}",
                explanation,
                aws_suggestions.join("\n")
            )
        }
    }
}
