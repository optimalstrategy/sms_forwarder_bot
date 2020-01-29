class SingletonType(type):
    """
    A Singleton metaclass.
    """

    def __call__(cls, *args, **kwargs):
        """
        Returns the instance if already exists, creates it otherwise.
        """
        try:
            return cls.__instance
        except AttributeError:
            cls.__instance = super(SingletonType, cls).__call__(*args, **kwargs)
            return SingletonType.__call__(cls, *args, **kwargs)

    def get_instance(cls):
        """
        Returns the existing class instance.

        :return: class instance
        """
        return cls.__instance
