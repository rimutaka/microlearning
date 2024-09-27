Select 3 features that apply to AWS Lambda Provisioned Concurrency setting.

# It incurs additional charges
Correct.
The charges are different compared to on-demand functions and include a fee for the every GB-second of provisioned functions regardless if they are running or not.

https://aws.amazon.com/lambda/pricing/#Provisioned_Concurrency_Pricing


# It limits how many Lambdas can run concurrently
Incorrect.
Invoking more than what was provisioned runs the extra instances as on-demand functions.

Reserved Concurrency setting controls how many functions can be invoked before the calls are throttled.

https://docs.aws.amazon.com/lambda/latest/dg/configuration-concurrency.html

# It affects the cold start time
Correct.

For functions using provisioned concurrency, Lambda runs any initialization code, such as loading libraries and instantiating clients, during allocation time. Therefore, it's advisable to move as much initialization outside of the main function handler to avoid impacting latency during actual function invocations.

# It allocates more CPU
Incorrect.
The CPU allocation depends on the amount of memory.

https://docs.aws.amazon.com/lambda/latest/dg/configuration-memory.html#configuration-memory-use-cases

# It allocates more storage
Incorrect.

Ephemeral storage is configured separately and is not affected by the Provisioned Concurrency setting.

https://docs.aws.amazon.com/lambda/latest/dg/configuration-ephemeral-storage.html

# It requires different code optimization compared to on-demand functions
Correct.

For functions using provisioned concurrency, Lambda runs any initialization code, such as loading libraries and instantiating clients, during allocation time. Therefore, it's advisable to move as much initialization outside of the main function handler to avoid impacting latency during actual function invocations.

https://docs.aws.amazon.com/lambda/latest/dg/provisioned-concurrency.html#optimizing-latency

# Only some runtimes support provisioned concurrency
Incorrect.

There is no limitation by runtime for provisioned concurrency,
but different runtimes will have different code optimization approaches to do most of the initialization during the cold start.

https://docs.aws.amazon.com/lambda/latest/dg/provisioned-concurrency.html#optimizing-latency