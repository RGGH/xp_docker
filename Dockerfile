FROM python:3.10

# Set the working directory
WORKDIR /app

# Copy the Python script and requirements.txt into the container
COPY ./hello_world.py /app/
COPY ./requirements.txt /app/

# Install dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Command to run the Python script
CMD ["python", "hello_world.py"]

