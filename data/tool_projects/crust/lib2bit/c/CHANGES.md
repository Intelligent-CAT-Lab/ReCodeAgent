0.2.2:
  - Fixed an error if the length requested is very short (issue #10)

0.2.1:
  - Fixed a segfault that occurs if a 1 or 2 base sequence is requested that does not overlap either the start or end of a byte.

0.2.0:
  - The `twobitBasesWorker()` and `constructSequence()` functions that underly `twobitSequence()` and `twobitBases()` have been rewritten and are now much faster.

0.1.0:
  - Initial release
