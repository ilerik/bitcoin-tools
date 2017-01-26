import redis
import sys

config = {
    'host': '172.17.0.1',
    'port': 6379,
    'db': 0,
}

rd = redis.Redis(**config)
ps = rd.pubsub()

if __name__ == '__main__':
    channel = sys.argv[1]
    message = sys.argv[2]
    rd.publish(channel, message)