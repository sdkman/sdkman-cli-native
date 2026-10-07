# Users see "SDK", code keeps "candidate"

User-facing text calls installable software an **SDK**. Code, the SDKMAN! API and metadata files such as `var/candidates` keep the word **candidate**. "Candidate" is established across the codebase, the API and the other SDKMAN! services, so renaming it would be costly and give users nothing. But it is jargon that new users have to have explained, while "SDK" matches the SDKMAN! name and is understood right away. Do not rename the code to "SDK", and do not let "candidate" leak into output. See [CONTEXT.md](../../CONTEXT.md).
