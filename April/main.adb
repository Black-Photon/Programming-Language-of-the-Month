with Ada.Text_IO;
with Ada.Numerics.Float_Random; use Ada.Numerics.Float_Random;
with get_rod_absorption;
with rod_control;
with types;

procedure Main is
    use types;

    Choice : Float; -- Random Number Choice (0 to 1)
    G : Generator;  -- Random Number Generator

    Temperature : Kilojoule := 0.001;
    Output : Kilojoule := 0.0;

    Control_Rods : Rod_Array;
    Absorption : Percent;

    Input  : String (1 .. 10);
    Last : Natural;
begin
    Reset(G);
    for i in (Index) loop
        Control_Rods(i) := 0.0;
    end loop;

Outer_Loop:
    loop
        rod_control(Control_Rods, Temperature, Output);

        Choice := Random(G);
        if Choice <= 0.03 then
            -- Temperature quadruples this second
            Temperature := Temperature * 4;
        elsif Choice < 0.97 then 
            -- Temperature doubles each second
            Temperature := Temperature * 2;
        end if;
        -- Otherwise Temperature doesn't change

        -- Apply control rod modifier
        Absorption := get_rod_absorption(Control_Rods);
        Temperature := Temperature * Kilojoule (1.0 - Absorption);

        -- 5% of Temperature dissipates each second
        Output := Temperature * 0.05;
        Temperature := Temperature * 0.95;

        Ada.Text_IO.Put_Line ("Heat is" & Kilojoule'Image(Temperature) & "KJ, Output of" & Kilojoule'Image(Output) & "KJ/s");
        delay 1.0;
        if Temperature >= 750.000 then
            Ada.Text_IO.Put_Line ("FATAL: Heat is above hard limit of 750KJ.");
            Ada.Text_IO.Put_Line ("       Plant should be evacuated.");
            exit Outer_Loop;
        elsif Temperature >= 500.000 then
            Ada.Text_IO.Put_Line ("Warning: Heat is above safety limit of 500KJ.");
        elsif Temperature = 0.000 then
            Ada.Text_IO.Put_Line ("Warning: Reactor heat is at 0KJ");
            loop
                Ada.Text_IO.Put ("         Would you like to resart? (Y/N) ");
                Ada.Text_IO.Get_Line (Input, Last);
                exit Outer_Loop when Input (1) = 'N' or Input (1) = 'n';
                exit when Input (1) = 'Y' or Input (1) = 'y';
            end loop;
            Ada.Text_IO.Put_Line ("");
            Temperature := 0.001;
        end if;
    end loop Outer_Loop;
end Main;