Create a recursive descent parser for the tokens.
Start with the following rules (tokens in <>):
Sentence = <Word>| Sentence <Space> <Word> <Punctuation>
Heading = Sentence <NewLine> <HeadingUnderline> | <Heading> <NewLine> Sentence <NewLine><HeadingUnderline><NewLine><BlankLine>
Comment = <DoubleDot> <Space> Sentence <NewLine>
<IndentedBlock> = (<Indent> (<Sentence>)+ <Newline>)+
Directive = <DoubleDot> <Space> <Word> <DoubleColon> <Sentence> <Newline> <IndentedBlock>
