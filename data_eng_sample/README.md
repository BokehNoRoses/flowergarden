# Overview
---
##### The Current Population Survey (CPS) is the source of the official government statistics on employment and unemployment. The CPS has been conducted monthly for over 60 years. Currently, we obtain interviews from about 54,000 households monthly, scientifically selected on the basis of area of residence to represent the nation as a whole, individual states, and other specified areas. Each household is interviewed once a month for four consecutive months one year, and again for the corresponding time period a year later. This technique enables us to obtain reliable month-to-month and year-to-year comparisons at a reasonable cost while minimizing the inconvenience to any one household.

##### Although the main purpose of the survey is to collect information on the employment situation, a very important secondary purpose is to collect information on demographic characteristics such as age, sex, race, marital status, educational attainment, family relationship, occupation, and industry. From time to time, additional questions are included on health, education, income, and previous work experience. The statistics resulting from these questions serve to update similar information collected once every 10 years through the decennial census, and are used by government policymakers and legislators as important indicators of our nation's economic situation and for planning and evaluating many government programs.

## Selection Criteria
1. `HRHHID` - Household ID (PRIMARY KEY)
2. `HRYEAR4` - Census Year (PRIMARY KEY)
3. `HURESPLI` - Interviewee Line Number (PRIMARY KEY)
4. `HETENURE` - Ownership Category Code
	- 01 - Owned
	- 02 - Rented
	- 03 - Squatted
5. `HEHOUSUT` - Housing Unit Type
	- 00|07|08|12 - Other
	- 01|04 - House
	- 02|03|09 - Hotel
	- 05|06 - Mobile
	- 10 - Tent
	- 11 - Dorm
6. `HEFAMINC` - Family Income
	- 01 - <5K
	- 02 - <7.5K
	- 03 - <10K
	- 04 - <12.5K
	- 05 - <15K
	- 06 - <20K
	- 07 - <25K
	- 08 - <30K
	- 09 - <35K
	- 10 - <40K
	- 11 - <50K
	- 12 - <60K
	- 13 - <75K
	- 14 - <100K
	- 15 - <150K
	- 16 - >=150K
7. `HRNUMHOU` - Number of Residents
8. `HRHTYPE` - Household Type
	- 00 - Other
	- 01-02 - Married
	- 03-08 - Unmarried
	- 09-10 - Multi-tenant
9. `GEDIV` - Geographic Division
	- 01 - New England
	- 02 - Middle Atlantic
	- 03 - East North Central
	- 04 - West North Central
	- 05 - South Atlantic
	- 06 - East South Central
	- 07 - West South Central
	- 08 - Mountain
	- 09 - Pacific
10. `GTMETSTA` - Metropolitan Status
	- 01 - Metropolitan
	- 02 - Nonmetropolitan
	- 03 - Other
## Evaluation Criteria
1. `HRFS12MD` - Detailed Food Security Status (12m)
	- 01 - High
	- 02 - Medium
	- 03 - Low
	- 04 - Critical
	- -9 - No Response
2. `HRFS30D2` - Detailed Food Security Status (30d)
	- 01 - High
	- 02 - Medium
	- 03 - Low
	- 04 - Critical
	- -9 - No Response


# Methodology
---
### 1. Import the dependencies and import the data
``` python
# Data imported via the `./ingest.sh` scripts compresses the datasets using the `xz` protocol.
# Luckily, pandas `read_csv` func implements a compression algorithm option and can handle just fine.

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from os import walk

sources = [file[2] for file in walk('./sources')][0]         # Walks the sources subdirectory and returns only the file names.
dfs = [pd.read_csv('./sources/' + file) for file in sources] # Since sources is a single-depth directory, this is fine for a basic
                                                             #   implementation.
df = pd.concat(dfs)                                          # Combines (5) years of census data into a single dataframe.

df # Yields 650810 rows x 534 columns. Yikes!
```
![[imported_data.png]]

### 2. Clean up the data
``` python
# We will need to restrict our scope as this dataset provides a large set of parameters to choose from.
# This is census data, so this is as much a findings document as it is an auditing document. We will find
# many entries in which surveys were unable to be conducted for numerous reasons, such as not being available
# at the time of the interview or the household has fallen into disrepair. Certainly, there are many insights
# to gain from parsing the metadata about the data source, but for this demo we want to focus on readily
# tangible attributes that positively correlate with food security, such as the place you live and the amount
# of money you make.

dt = df[[
    'HRHHID',   # Household ID (PRIMARY KEY)
    'HRYEAR4',  # Census Year (PRIMARY KEY)
    'HURESPLI', # Interviewee Line Number (PRIMARY KEY)
    'HETENURE', # Ownership Category Code
    'HEHOUSUT', # Housing Unit Type
    'HEFAMINC', # Family Income
    'HRNUMHOU', # Number of Residents
    'HRHTYPE',  # Household Type
    'GEDIV',    # Geographic Division
    'GTMETSTA', # Metropolitan Status
    'HRFS12MD', # Detailed Food Security Category (12m) (EVALUATION)
    'HRFS30D2', # Detailed Food Security Category (30d) (EVALUATION)
]] \
.drop_duplicates(subset=['HRHHID', 'HRYEAR4', 'HURESPLI']) \
.set_index(['HRHHID', 'HRYEAR4', 'HURESPLI']) # This dataset requires a rather complex composite key to uniquely identify rows.
                                              #   We essentially need to say in order to retrieve a specific entry:
                                              #   "Give me the result of the interview with {X} interviewee during {Y} year
                                              #   from {Z} household." Otherwise, we run into the issue of multiple
                                              #   representations in the dataset.

# Through manual exploration of the data, this one-liner removes many of `NaN` plaguing the dataset since it includes
# results from other than completed interviews.
dt.drop(dt.loc[dt['HRNUMHOU'] < 1].index, inplace=True)

dt # Yields 237153 rows x 9 cols. Much more manageable!
```
![[cleaned_data.png]]

### 3|4. Load the data
``` python
import sqlalchemy as sa                          # Provides a way of interacting with databases 
                                                 #   through python and integrates with pandas
from sqlalchemy.types import BigInteger, Integer # Import DB types to use for schema creation
# import ssl                                     # Used if our DB is TLS enabled
import pg8000                                    # specific dialect to use alongside postgresql that supports TLS
from dotenv import dotenv_values                 # Import our secrets

# NOTE: this file and `pgpass` would not be included in a `git push`!
# For demo purposes, it is included to showcase the project structure.
secrets = dotenv_values('.env')                  # Using a .env file with locked down permissions to proivde rudimentary security 
                                                 #   for connection strings

# For SSL support with the pg8000 dialect
# ssl_context = ssl.create_default_context()
# ssl_context.check_hostname = False         # Only needed for self-signed certificates
# ssl_context.verify_mode = ssl.CERT_NONE    # Only needed for self-signed certificates

# Create the SQL to use with our DB. Here we will use postgresql to store the cleaned results.
engine = sa.create_engine(
    secrets['CONN_STR'],    # postgresql+pg8000://user:password@127.0.0.1:5432/load
    client_encoding='utf-8' # pg8000 processes entries in UNICODE
    # connect_args={"ssl_context": ssl_context}
)

# Pandas provides a convenience function for interaction with DB through the SQLAlchemy library.
# Here we establish the primary keys through the use of `index_labels` and define the schema
# through the `dtypes` attribute.

# It may be more desirable for us to SQLAlchemy to define the schema through the `MetaData` obj
# to create our table as we can then setup foreign keys for each field reference per the census'
# technical documentation. One such field reference that could be highly reusable is `GESTFIPS`
# as this is a national standard coding convention that could used across a great many datasets.

# Additionally, if desired, we could store the transformed fields instead for ease of use and 
# readability. Ultimately, it would be a design decision, such as determining whether VARCHAR
# would be sensible versus the possible space trade-off against INT types. For this demo, we
# will keep it simple and load just the cleaned result to demonstrate the concept.
dt.to_sql(
    'load',
    engine,
    index_label=['HRHHID', 'HRYEAR4', 'HURESPLI'],
    dtype={
        'HRHHID': BigInteger(),
        'HRYEAR4': Integer(),
        'HURESPLI': Integer(),
        'HETENURE': Integer(),
        'HEHOUSUT': Integer(),
        'HEFAMINC': Integer(),
        'HRHTYPE': Integer(),
        'GEDIV': Integer(),
        'GTMETSTA': Integer(),
        'HRFS12MD': Integer(),
        'HRFS30D2': Integer()
    }
)
```

### 3|4. Define and apply the transforms
``` python
# We can do these transforms as additional tables in the DB if we so choose:

# from sqlalchemy import MetaData, Table, Column, Integer, String
# md = MetaData()
# occ_transform = Table (
#    "occ",
#    md,
#    Column("case", Integer, primary_key=True, autoincrement=False),
#    Column("val", String(10), nullable=False)
# )
# occ_transform.create(engine)
# with engine.connect() as conn:
#   conn.execute(insert(), {"case": $CASE_NUM, "occ_str": $CASE_VAL}
#   ...
#   conn.commit()

# In `load` table:
# load = Table (
#   ...
#   Column("HETENURE", Integer, ForeignKey("occ.case") ...)
# )

# Then we can select results like, where the select command can be input
# as a sqlalchemy `text` obj to `conn.execute()`:
# SELECT load.HRHHID, occ.occ_str
# FROM load
# INNER JOIN load ON load.HETENURE=occ.case

def occ_transform(val):
    match val:
        case 1:
            return 'Owned'
        case 2:
            return 'Rented'
        case 3:
            return 'Squatted'
        case _:
            return 'Other'

...

def hrfs_transform(val):
    match val:
        case 1:
            return 'High'
        case 2:
            return 'Medium'
        case 3:
            return 'Low'
        case 4:
            return 'Critical'
        case -9:
            return 'No Response'
        case _:
            return 'Unknown'

dt['HETENURE'] = dt['HETENURE'].apply(occ_transform)
...
dt['HRFS12MD'] = dt['HRFS12MD'].apply(hrfs_transform)
dt['HRFS30D2'] = dt['HRFS30D2'].apply(hrfs_transform)
```

### 5. Visualize the results
``` python
sum_hrfs12md_by_year = dt[(dt['HRFS12MD'] != 'Unknown') & (dt['HRFS12MD'] != 'No Response')]['HRFS12MD'] \
.groupby(level='HRYEAR4') \
.value_counts() \
.reset_index() # For ease of index slicing, we reset the index since only 20 rows are going to be present

years = np.array(sorted(dt.index.get_level_values('HRYEAR4').unique())) # array([2019, 2020, ...])
ind = sum_hrfs12md_by_year.groupby(by='HRFS12MD').indices               # ['Critical', 'High', ...]

summarize = {} # This will invert the frame so that we can reference each category code across all census years.
               # i.e.: { 'Critical': [2019val, 2020val, ...] }
for i in ind:
    summarize[i] = sum_hrfs12md_by_year.loc[ind[i]]['count'].to_numpy()

width = 0.5
fig, ax = plt.subplots()
bottom = np.zeros(len(years))
for idx, val in summarize.items():
    p = ax.bar(years, \
               val, \
               width, \
               label=idx, \
               bottom=bottom)
    bottom += val
    ax.bar_label(p, label_type='center')

ax.set_ylabel('Count (Food Security Category Codes)')
ax.set_xlabel('Census Year')
ax.set_title('Food Security Over The Years')
ax.legend(loc='upper left', ncols=4)
ax.set_ylim(0, 40000)
plt.show()

# Here we see some interesting results:
# 1. Prior to the pandemic, census surveys held consistent with relatively unfluctuating numbers.
#      Even after 9 months of the declared emergency, the reports show relatively unchanging results.
#      This possibly alludes to the fact that they surveys are (1) conducted fairly early into the year
#      or (2) republished if the interviewee does not reply after successfully completing a previous
#      year's interview or replies with no change. However, we can see over the previous (3) years
#      that census surveys are rebounding to pre-pandemic total counts.
# 2. In the past (3) census years, we've seen an increasing de-stabilization of food security on
#      a micro-level with marginal, low, and critical food security scores increasing on average
#      28% between 2021 and 2022 and 7% between 2022 and 2023. These probably have additional 
#      indicators with the pandemic work environment and Russo-Ukrainian War respectively as the sharp 28% 
#      increase is likely due to more government workers being able to conduct the census than in 2021.
#      The actual might be closer to 5-7% for the total American population.

# For government or business purposes, we glean additional data sources that may be required to augment
# this set if our priority was to combat food insecurity. We might ask ourselves:
# 1. If wars are contributing to the 5-7% increased risk in food security, where are the most affected
#      populaces? This question might be answered by this dataset as we can drill down into regional,
#      subregional, state, and county geography for census results and correlate it with another dataset
#      for food exports/imports by whatever level of geographic granularity we need. KPIs in this case
#      might be things such as outreach engagement (i.e. food banks, NGOs, ...) or sustainable living
#      practices (i.e. increasing agricultural production, disaster supplies, ...).
```
![[results_graph.png]]
