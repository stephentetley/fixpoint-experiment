

import java.sql.*;
import java.sql.Connection;
import java.sql.Statement;

public class DeliveryDate {


    public static void main(String[] args) throws Exception {
        System.out.println("Running...");
        deliveryDate();
    }

    private static void deliveryDate() throws Exception {
        Connection conn = DriverManager.getConnection("jdbc:duckdb:data/delivery_date.duckdb");
        Statement stmt = conn.createStatement();
        String query = "";
        int deltaReadyDateCount = 0;
        int deltaZresultCount = 0;
        stmt.execute("CREATE OR REPLACE TABLE part_depends (part VARCHAR, component VARCHAR);");
        stmt.execute("CREATE OR REPLACE TABLE assembly_time (part VARCHAR, days INTEGER);");
        stmt.execute("CREATE OR REPLACE TABLE delivery_date (component VARCHAR, days INTEGER);");
        stmt.execute("CREATE OR REPLACE TABLE ready_date (part VARCHAR, days INTEGER, PRIMARY KEY(part));");
        stmt.execute("CREATE OR REPLACE TABLE delta_ready_date (part VARCHAR, days INTEGER, PRIMARY KEY(part));");
        stmt.execute("CREATE OR REPLACE TABLE new_ready_date (part VARCHAR, days INTEGER, PRIMARY KEY(part));");
        stmt.execute("CREATE OR REPLACE TABLE zresult (part VARCHAR, days INTEGER, PRIMARY KEY(part));");
        stmt.execute("CREATE OR REPLACE TABLE delta_zresult (part VARCHAR, days INTEGER, PRIMARY KEY(part));");
        stmt.execute("CREATE OR REPLACE TABLE new_zresult (part VARCHAR, days INTEGER, PRIMARY KEY(part));");


        stmt.execute("INSERT INTO part_depends (part, component) VALUES ('Car', 'Chassis'), ('Car', 'Engine'), ('Engine', 'Piston'), ('Engine', 'Ignition');");
        stmt.execute("INSERT INTO assembly_time (part, DAYS) VALUES ('Car', 7), ('Engine', 2);");
        stmt.execute("INSERT INTO delivery_date (component, DAYS) VALUES ('Chassis', 2), ('Piston', 1), ('Ignition', 7);");

        // ReadyDate(VarSym(part); VarSym(date)) :- DeliveryDate(VarSym(part); VarSym(date)).;
        query = """
                INSERT INTO ready_date(part, days)
                SELECT
                    component AS part,
                    days AS days,
                FROM delivery_date;
            """;
        stmt.execute(query);

        // ReadyDate(VarSym(part); <clo>(VarSym(componentDate), VarSym(assemblyTime))) :- PartDepends(VarSym(part), VarSym(component)), AssemblyTime(VarSym(part), VarSym(assemblyTime)), ReadyDate(VarSym(component); VarSym(componentDate)).;
        query = """
                INSERT INTO ready_date(part, days)
                SELECT
                    t0.part AS part,
                    max(t1.days + t2.days) AS days,
                FROM
                    part_depends t0,
                    assembly_time t1,
                    ready_date t2,
                WHERE t1.part = t0.part AND t2.part = t0.component
                GROUP BY t0.part
            """;
        stmt.execute(query);

        stmt.execute("INSERT INTO delta_ready_date (part, days) SELECT part, days FROM ready_date;");

        // loop - use a vacuous condition, actual condition tested for before the `break` statement
        while(true) {
            // purge new_ready_date
            stmt.execute("DELETE FROM new_ready_date;");

            // ReadyDate(VarSym(part); VarSym(date)) :- DeliveryDate(VarSym(part); VarSym(date)).;
            query = """
                    INSERT INTO new_ready_date(part, days)
                    SELECT
                        t0.component AS part,
                        t0.days AS days,
                    FROM delivery_date t0
                    EXCEPT
                    SELECT
                        t1.part AS part,
                        t1.days AS days,
                    FROM ready_date t1
                """;
            stmt.execute(query);

            query = """
                    INSERT INTO new_ready_date(part, days)
                    SELECT
                        t0.part AS part,
                        max(t1.days + t2.days) AS days,
                    FROM
                        part_depends t0,
                        assembly_time t1,
                        ready_date t2,
                    WHERE t1.part = t0.part AND t2.part = t0.component
                    GROUP BY t0.part
                    EXCEPT
                    SELECT
                        t3.part AS part,
                        t3.days AS days,
                    FROM ready_date t3
                """;
            stmt.execute(query);

            // merge new_ReadyDate into ReadyDate;
            stmt.execute("INSERT INTO ready_date (part, days) SELECT part, days FROM new_ready_date ON CONFLICT DO UPDATE SET days = EXCLUDED.days;");
            swap("new_ready_date", "delta_ready_date", conn);

            deltaReadyDateCount = countTuples("delta_ready_date", conn);
            System.out.println(String.format("loop - deltaReadyDateCount: %d", deltaReadyDateCount));
            if (deltaReadyDateCount <= 0) {
                break;
            }
        }

        // calc zresult...
        query = """
            INSERT INTO zresult(part, days)
            SELECT
                part AS part,
                days AS days,
            FROM ready_date;
        """;
        stmt.execute(query);
        // merge $Result into delta_$Result;
        stmt.execute("INSERT INTO delta_zresult (part, days) SELECT part, days FROM zresult ON CONFLICT DO UPDATE SET days = EXCLUDED.days;");

        while(true) {
            // purge new_$Result;
            stmt.execute("DELETE FROM new_zresult;");

            query = """
                    INSERT INTO new_zresult(part, days)
                    SELECT
                        t0.part AS part,
                        t0.days AS days,
                    FROM ready_date t0
                    EXCEPT
                    SELECT
                        t1.part AS part,
                        t1.days AS days,
                    FROM zresult t1
                """;
            stmt.execute(query);

            // merge new_$Result into $Result;
            stmt.execute("INSERT INTO zresult (part, days) SELECT part, days FROM new_zresult ON CONFLICT DO UPDATE SET days = EXCLUDED.days;");
            swap("new_zresult", "delta_zresult", conn);

            deltaZresultCount = countTuples("delta_zresult", conn);
            System.out.println(String.format("loop - deltaZresultCount: %d", deltaZresultCount));
            if (deltaZresultCount <= 0) {
                break;
            }
        }

        // print results
        System.out.println("zresult");
        ResultSet rs = stmt.executeQuery("SELECT part, days FROM zresult;");
        while(rs.next())
        {
            System.out.println(String.format("part: %s, days: %d", rs.getString("part"), rs.getInt("days")));
        }

        conn.close();
    }


    private static void swap(String table1, String table2, Connection conn) throws Exception {
        String table_swap = String.format("%s_swap", table1);
        Statement stmt = conn.createStatement();
        stmt.execute(String.format("ALTER TABLE %s RENAME TO %s;", table1, table_swap));
        stmt.execute(String.format("ALTER TABLE %s RENAME TO %s;", table2, table1));
        stmt.execute(String.format("ALTER TABLE %s RENAME TO %s;", table_swap, table2));
    }

    private static int countTuples(String table, Connection conn) throws Exception {
        Statement stmt = conn.createStatement();
        ResultSet rs = stmt.executeQuery(String.format("SELECT COUNT(*) FROM %s;", table));
        if(rs.next()) {
            return rs.getInt(1);
        } else {
            return 0;
        }
    }
}
