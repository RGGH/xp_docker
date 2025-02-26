FROM python:3.10

# Set the working directory
WORKDIR /app

# Copy the Python script and requirements.txt into the container
COPY ./btc_price.py /app/
COPY ./requirements.txt /app/

# Install dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Command to run the Python script
CMD ["python", "btc_price.py"]

