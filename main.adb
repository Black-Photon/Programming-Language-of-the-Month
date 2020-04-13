with Ada.Text_IO;
with Ada.Numerics.Float_Random; use Ada.Numerics.Float_Random;

procedure Main is
    type Kilojoule is delta 10.0 ** (-4) digits 8;
    Choice : Float;
    G : Generator;
    Temperature : Kilojoule := 0.001;
    Output : Kilojoule;
begin
    Reset(G);
    loop
        Choice := Random(G);
        if Choice <= 0.03 then
            -- Temperature quadruples this second
            Temperature := Temperature * 4;
        elsif Choice < 0.97 then 
            -- Temperature doubles each second
            Temperature := Temperature * 2;
        end if;
        -- Otherwise Temperature doesn't change

        -- 5% of Temperature dissipates each second
        Output := Temperature * 0.05;
        Temperature := Temperature * 0.95;

        Ada.Text_IO.Put_Line ("Heat is" & Kilojoule'Image(Temperature) & "KJ, Output of" & Kilojoule'Image(Output) & "KJ/s");
        delay 1.0;
        if Temperature >= 750.000 then
            Ada.Text_IO.Put_Line ("FATAL: Heat is above hard limit of 750KJ.");
            Ada.Text_IO.Put_Line ("       Plant should be evacuated.");
            exit;
        elsif Temperature >= 500.000 then
            Ada.Text_IO.Put_Line ("Warning: Heat is above safety limit of 500KJ.");
        end if;
    end loop;
end Main;