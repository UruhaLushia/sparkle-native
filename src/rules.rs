use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct RuleConvertOptions {
    pub input_target: Option<String>,
    pub input_format: Option<String>,
    pub input_behavior: Option<String>,
    pub output_target: Option<String>,
    pub output_format: Option<String>,
    pub output_behavior: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RuleOutputInfo {
    pub behavior: Option<String>,
    pub format: String,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub struct RuleSkippedItem {
    pub rule: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct RuleStringResult {
    pub kind: String,
    pub outputs: HashMap<String, String>,
    pub info: HashMap<String, RuleOutputInfo>,
    pub skipped: Vec<RuleSkippedItem>,
}

pub fn rule_file_to_string(
    path: impl AsRef<Path>,
    options: RuleConvertOptions,
) -> anyhow::Result<RuleStringResult> {
    use rule_converter::{
        BehaviorMode, ConvertOptions, FileInput, InputBehaviorMode, InputFormat, OutputFormat,
        RuleTarget, convert_file_inputs, default_output_behavior, write_outputs_as_to_memory_owned,
    };

    let input_target = options
        .input_target
        .as_deref()
        .map(RuleTarget::parse_arg)
        .transpose()?;
    let input_format = options
        .input_format
        .as_deref()
        .map(InputFormat::parse_arg)
        .transpose()?;
    let input_behavior = options
        .input_behavior
        .as_deref()
        .map(InputBehaviorMode::parse_arg)
        .transpose()?
        .unwrap_or(InputBehaviorMode::Auto);
    let output_target = options
        .output_target
        .as_deref()
        .map(RuleTarget::parse_arg)
        .transpose()?
        .unwrap_or(RuleTarget::Mihomo);
    let output_format = options
        .output_format
        .as_deref()
        .map(OutputFormat::parse_arg)
        .transpose()?
        .unwrap_or(OutputFormat::Text);
    let output_behavior = options
        .output_behavior
        .as_deref()
        .map(BehaviorMode::parse_arg)
        .transpose()?
        .unwrap_or_else(|| default_output_behavior(output_target, output_format));

    let convert_options = ConvertOptions {
        input_target,
        input_format,
        input_behavior,
        output_target,
        output_format,
        output_behavior,
    };
    let result = convert_file_inputs(
        [FileInput {
            path: path.as_ref().to_path_buf(),
            target: input_target,
            format: input_format,
            behavior: input_behavior,
        }],
        convert_options,
    )?;
    let (outputs, skipped) =
        write_outputs_as_to_memory_owned(result, output_target, output_format)?;

    let mut output_values = HashMap::with_capacity(outputs.len());
    let mut output_info = HashMap::with_capacity(outputs.len());
    for output in outputs {
        let name = output.behavior.as_str().to_string();
        let text = String::from_utf8(output.bytes)
            .map_err(|error| anyhow::anyhow!("output {name} is not valid UTF-8: {error}"))?;

        output_info.insert(
            name.clone(),
            RuleOutputInfo {
                behavior: Some(output.behavior.as_str().to_string()),
                format: output.format.as_str().to_string(),
                count: output.count as u32,
            },
        );
        output_values.insert(name, text);
    }

    Ok(RuleStringResult {
        kind: "rules".to_string(),
        outputs: output_values,
        info: output_info,
        skipped: skipped
            .into_iter()
            .map(|item| RuleSkippedItem {
                rule: item.rule,
                reason: item.reason,
            })
            .collect(),
    })
}
