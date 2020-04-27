package types is
    type Percent is delta 10.0 ** (-2) digits 5;
    type Index is range 1 .. 100;
    type Rod_Array is array (Index) of Percent;
    type Kilojoule is delta 10.0 ** (-4) digits 8;
end types;
