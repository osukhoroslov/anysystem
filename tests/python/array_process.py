from array import array

from anysystem import Context, Message, Process


class ArrayProcess(Process):
    def __init__(self):
        self.data = array('Q')

    def on_start(self, ctx: Context):
        pass

    def on_local_message(self, msg: Message, ctx: Context):
        if msg.type == 'FILL':
            self.data = array('Q', [2**40]) * 10240
        elif msg.type == 'CLEAR':
            self.data = array('Q')

    def on_message(self, msg: Message, sender: str, ctx: Context):
        pass

    def on_timer(self, timer_name: str, ctx: Context):
        pass
