import csv
from zxcvbn import zxcvbn

rockyoupath = "data/unprocessed/rockyou.txt"
outputpath = "data/processed/processedinfo.csv"

def process_passwords(inputfile, outputfile, batch_size=5000):

    with open(inputfile, "r", encoding="utf-8", errors="ignore") as infile_temp:
        line_count = sum(1 for _ in infile_temp)

    with open(inputfile, "r", encoding="utf-8", errors="ignore") as infile, \
        open(outputfile, "w", newline="", encoding="utf-8") as outfile:
        writer = csv.writer(outfile, delimiter="\t")

        writer.writerow(["password", "score", "crack_time"])

        batch = []

        for i, line in enumerate(infile):
            password = line.strip()
            if not password:
                continue

            try:
                result = zxcvbn(password)
                score = result["score"]
                crack_time = result["crack_times_display"]["online_no_throttling_10_per_second"]

                batch.append([password, score, crack_time])

            except Exception as e:
                print(f"Error processing password: {password}, Error: {e}")
                continue

            if len(batch) >= batch_size:
                writer.writerows(batch)
                batch = []

            if i % 5000 == 0:

                print(f"Processed {i}/{line_count} passwords ({round((i/line_count) * 100, 1)}%)")


        if batch:
            writer.writerows(batch)

    print("Processing complete. Results saved to: ", outputfile)

process_passwords(rockyoupath, outputpath)
