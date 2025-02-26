# Use the Python 3.10 base image
FROM python:3.10

# Set the working directory inside the container
WORKDIR /app

# Copy both the Python script and the requirements file into the container
COPY hello_world.py /app/
COPY requirements.txt /app/

# Install the Python dependencies from requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Command to run the Python script
CMD ["python", "hello_world.py"]

