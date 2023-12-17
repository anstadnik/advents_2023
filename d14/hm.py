import plotly.express as px

with open("hm.txt", "r") as f:
    # print(f.readlines())
    data = [int(i) for i in f.readlines()]

# print(data)

px.line(y=data).show()
