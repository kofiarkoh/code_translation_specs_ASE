#include <stdio.h>
#include <stdlib.h>

int main(void) {
    char *buf;
    char *p;
    int i;
    char before;
    int cnt = 0;
    int fin = 0;

    buf = malloc(100003);
    p = fgets(buf, 100003, stdin);

    before=buf[0];
    for(i=1;;i++){
        switch(buf[i]) {
            case 'B':
            case 'W':
                if(before != buf[i]){
                    before = buf[i];
                    cnt++;
                }
                break;
            default:
                fin = 1;
        }

        if(fin) break;
    }

    printf("%d\n", cnt);
    free(buf);

    return 0;
}
