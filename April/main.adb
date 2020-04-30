with Ada.Text_IO;
with Ada.Numerics.Float_Random; use Ada.Numerics.Float_Random;
with get_rod_absorption;
with rod_control; use rod_control;
with types;
with signal; use signal;

procedure Main is
    use types;

    Choice : Float; -- Random Number Choice (0 to 1)
    G : Generator;  -- Random Number Generator

    Temp_Potential : Kilojoule := 0.001;    -- Temperature potential is the total neutron energy in the system
    Temperature : Kilojoule := 0.000;       -- Temperature is the absolute energy in the system (not inc neutrons)
    Output : Kilojoule := 0.0;

    Control_Rods : Rod_Array;
    Absorption : Percent;

    Input  : String (1 .. 10);
    Last : Natural;


    Sigint : Boolean := False;
    Term : Boolean := False;

    task Sig_Handler;
    task body Sig_Handler is
    begin
        Handler.Wait;
        if not Term then
            Sigint := True;
            Ada.Text_IO.Put_Line("Exit Control Signal Received. Shutting down reactor...");
            for i in (Index) loop
                Control_Rods(i) := 1.0;
            end loop;
            while Temperature > 0.0 and not Term loop null; end loop;
            Ada.Text_IO.Put_Line("Shutdown Success");
            Term := True;
        end if;
    end Sig_Handler;
begin
    Reset(G);
    for i in (Index) loop
        Control_Rods(i) := 0.0;
    end loop;

Outer_Loop:
    while not Term loop
        -- Ignore AI input for manual shutdown
        if not Sigint then
            callback(Control_Rods, Temperature, Output);
        end if;

        Choice := Random(G);
        if Choice <= 0.03 then
            -- Temperature potential quadruples this second
            Temp_Potential := Temp_Potential * 4;
        elsif Choice < 0.97 then 
            -- Temperature potential doubles each second
            Temp_Potential := Temp_Potential * 2;
        end if;
        -- Otherwise Temperature doesn't change

        -- Apply control rod modifier
        Absorption := get_rod_absorption(Control_Rods);
        Temp_Potential := Temp_Potential * Kilojoule (1.0 - Absorption);

        -- Apply Temperature Potential to Temperature
        Temperature := Temperature + Temp_Potential;

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
        elsif Temperature = 0.000 and not Sigint then
            Ada.Text_IO.Put_Line ("Warning: Reactor heat is at 0KJ");
            loop
                Ada.Text_IO.Put ("         Would you like to resart? (Y/N) ");
                Ada.Text_IO.Get_Line (Input, Last);
                exit Outer_Loop when Input (1) = 'N' or Input (1) = 'n';
                exit when Input (1) = 'Y' or Input (1) = 'y';
            end loop;
            Ada.Text_IO.Put_Line ("");
            Temp_Potential := 0.001;
        end if;
    end loop Outer_Loop;
    Term := True;
end Main;