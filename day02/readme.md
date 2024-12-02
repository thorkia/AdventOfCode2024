Day 2 Part 1

We have a list of reports - each report is a list of numbers.

Valid reports are either all increasing, or all decreasing and the difference needs to be >=1 && <=3

We will take each line and split it into a list of numbers - then itertate starting at index 1 until the end.  If we discover a not valid we will break out



Day 2 Part 2

For part 2, we still need to check part 1.  If it's a valid report, we do nothing.

If it's not, we need to apply the dampener and remove a single invalid item from the list.

If there is more than one invalid item, then the list is invalid - so we only need to bypass the first invalid item.

We can do this by keeping a dampener boolean that is set to false.  The first time we find an invalid item, we set it to true.

Then the next invalid item will stop the analysis and set it to false.