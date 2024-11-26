#include <pthread.h>
#include <stdio.h>

static pthread_key_t key;

void destr(void *ptr) {
    printf("destructor called for %p\n", ptr);
}

void *thread_fn(void *arg) {
    printf("thread start\n");
    pthread_setspecific(key, (void *)2);
    printf("thread exit\n");
    return NULL;
}

int main(int argc, char *argv[]) {
    printf("creating key\n");
    pthread_key_create(&key, &destr);

    printf("setting main value\n");
    pthread_setspecific(key, (void *)1);

    printf("starting thread\n");
    pthread_t thread;
    pthread_create(&thread, NULL, &thread_fn, NULL);

    printf("joining thread\n");
    pthread_join(thread, NULL);

    printf("exiting\n");
    return 0;
}