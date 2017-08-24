# ALICE-rs

This repository demonstrates how the public data released by the the CERN based ALICE collaboration can be analysed with the Rust programming language and with minimal further external dependencies.

**This is not an official ALICE or CERN project**

This collection of crates provides and demonstrates:

* A library/binary for downloading a desired amount of the publicly released data. See `alice-open-data`.
* Reading of so-called Event-summary-data (ESD) from [ROOT](https://root.cern.ch/)'s binary file format
  This is achieved through automatically generated bindings from ROOT (MakeClass) as well as from the Rust side (bindgen). See `alice-sys`.
* A safe wrapper around a limited subset of the automatically generated bindings.
  These bindings enable a convinient asynchronous IO and re-organize the data with a rustic interface. Alternative backends like databases would be integrated at this this level. See `alice`
* High performace n-dimensional histograms for streaming data
  Maintains a binned count of the data which can be successivly filled. See `histogram`
* Example analysis reproducing published results using all of the above crates. See `analyses`

## Dependencies

There are no dependencies on any ALICE specific software. However, the public data was released in the [ROOT](https://root.cern.ch/) framework's binary data format. Therefore, a working ROOT 5 or 6 installation is required for the IO process.

Furthermore, various excellent crates from the rust ecosystem have been leveraged. Most notably gcc-rs, bindgen, and ndarray.


## CERN, LHC, and ALICE

ALICE (A Large Ion Collider Experiment) is the dedicated Heavy Ion experiment at the Large Hadron Collider (LHC) in Switzerland. Just like most other CERN based experiments, its goal is to better understand the fundamental particles and forces of the universe. In particular, ALICE is concerned with the so called strong force which is the dominant actor for processes within an atomic nuclei. Many of the properties of this force manifest them self at extreme pressures and temperatures as they were found micro seconds after the Big Bang. By colliding heavy nuclei such as lead ions at nearly the speed of light, we are able to recreate such extreme conditions for a very brief moment within the ALICE detector. By carefully studying the particles produced at such collisions we can deduce the properties of the strong force which will help us to better understand nuclear reactions, neutron stars, the first moments of the universe and much more.

## CERN open data

ALICE, as well as some other CERN based experiments have released a small subset of their recorded data into the public domain. The dataset in question for this project is in total approximately 6.5TB. However, only 10-100 GB are necessary to meaningfully run the example analyses in this project.


## ALICE-rs vs. the standard c++ software stack

The state of the software stack in large scientific collaborations is often "challenging". Most particle physics experiments are build around ROOT, a c++ framework with all but the kitchen sink and 20 years of legacy. While ROOT has pioneered some great ideas in the past it has certainly taken some bad design choices as well. Raw pointers are omnipresent, every object inherits through sometimes 20 layers from the very same "TObject", and no member and no function is ever marked `const`. The ALICE software stack sits on top of this. Our framework has grown over decades with literally thousands of authors with very little enthusiasm for best practices of software engineering. The ALICE basic software stack (AliRoot) clocks more than 2.5 million lines of code alone. This does not include any actually analysis of the recorded data, just reconstructions from raw signals, simulations and "basic" classes. The collection of analyses is by now of equivalent size and goes by the name AliPhysics. Documentation is sparse throughout.

--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
 C++                   4002      2036430       297687       415642      1323101
 Plain Text             626       712452         6059            0       706393
 FORTRAN Legacy         467       866516        52435       151867       662214
 C/C++ Header          4244       530311       103766       139635       286910
 C                     2100       416533        54256        82426       279851
 PHP                    351       229509        34901         1405       193203
 XML                    355       193478        19048           84       174346
 HTML                   357       197991        33719          211       164061
 JSON                     1        19402            0            0        19402
 TeX                     47        22949         3634          727        18588
 Makefile               107        10053         1495          885         7673
 Bourne Shell           166         9550         1308         1903         6339
 Autoconf                27         4107          751         1878         1478
 Python                   4         1633          333          235         1065
 CUDA                     1         1147          154           57          936
 CSS                      8          616          104            0          512
 Markdown                 4          471           72            0          399
 C Shell                  3          382           22           42          318
 Perl                     2          327           30          101          196
 ASP.NET                  4            8            0            0            8
 Assembly                 1            8            0            0            8
--------------------------------------------------------------------------------
 Total                12877      5253873       609774       797098      3847001
--------------------------------------------------------------------------------
Lines of code of AliRoot; some files are clearly mis-identified, but the gist becomes clear.

AliRoot and AliPhysics are the officially recommended software to analyze public data. I believe that requiring the use of such a gigantic software stack from outside users is an almost impossible burden.

This project does not use a single line of code from AliRoot nor AliPhysics. Instead, it reads in the data as (more) primitive types and just implements the functions which are directly needed to select suitable collisions and produced particle "tracks". The code base is very lean in comparison:

--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
 C/C++ Header             1         1916           23           56         1837
 Rust                    22         1857          195          128         1534
 C++                      1          954           15           20          919
--------------------------------------------------------------------------------
 Total                   24         4727          233          204         4290
--------------------------------------------------------------------------------
Lines of code in this project

The c++ files are automatically generated and only slightly tweaked. The rust code includes everything needed to run a real world analysis as well as an implementation of a histogram for streaming data and a downloader for the data itself. Performance aside, I dare to call this an improvement to the status quo.


## Performance

I have discovered throughout the last weeks and months that one really does not have to sacrifice performance for convenience when using Rust!

It is difficult to write a true apples to apples bench mark, though.
Running any analysis through AliRoot implicitly does many more things which you might or might not want.
With this caveat aside, it is safe to say that ALICE-rs is faster than the standard AliRoot/AliPhysics stack by at least a factor of 6. Three main points are mainly responsible for this.

*(Given that I am very new to Rust, I am sure that this design can still be improved and I would be very greatful for any such tips and recommendations!)*

### Reading only the data needed
AliRoot always reads in all the data of a given collision instead of selecting only the "columns" of interest. This is probably due to a monolithic design choice. The data is exposed to the user in one huge "event" object which encompasses all possible data associated with the current event. Its all or nothing.
This project reads in only the data directly needed for the analysis. It then composes an
"event" struct.
Possibly missing data can easily be handled with Rust's `Option` enum.
Clearly some similar design could have been chosen in (modern) c++ as well.
It is still a testament to the benefits of having data in simple composable structs.


### Having a parallel io thread with a FIFO buffer
In the standard framework, each collision is read in from disk and processed sequentially. ALICE-rs does these two things in separate threads using the MPSC model from Rust's standard library which even comes with a FIFO buffer for free.


### A high performance n-dimensional histogram
This project also implements a n-dimensional histogram, which appears to be significantly faster than ROOT's alternative. This becomes especially important if the histogram has to be filled in nested loops.
