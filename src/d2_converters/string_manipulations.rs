pub fn strip_asterisk(line: &str) -> String {
    return line.replace('*', "");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn strip_asterisk_remove_all_asterisks(){
        let source = "*1*1**1*";
        let expected = "111";

        let actual = strip_asterisk(source);
        assert_eq!(actual, expected);
    }

    #[test]
    fn strip_asterisk_not_impact_input_with_no_asterisks(){
        let source = "i5guyfkbjghi85eigusrh589#@PR^Y{ERY}^";
        let expected = "i5guyfkbjghi85eigusrh589#@PR^Y{ERY}^";

        let actual = strip_asterisk(source);
        assert_eq!(actual, expected);

    }
}