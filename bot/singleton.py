class SingletonType(type):
    """
    A Singleton metaclass.
    """
    def __call__(cls, *args, **kwargs):
        """
        Returns instance if it's already exist, creates it otherwise.

        :param args: class args
        :param kwargs: class kwargs
        :return: class instance
        """
        try:
            return cls.__instance
        except AttributeError:
            cls.__instance = super(SingletonType, cls).__call__(*args, **kwargs)
            return SingletonType.__call__(cls, *args, **kwargs)

    def get_instance(cls):
        """
        Returns existing class instance.
        :return: class instance
        """
        return cls.__instance
