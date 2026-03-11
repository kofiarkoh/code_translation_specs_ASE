#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main(void) {
    char *buf;
    char *p;

    int pcnt = 0,gcnt = 0;
    int i = 0;

    buf = calloc(1,100002);

    p = fgets(buf, 100002, stdin);
    while(1){
        if(buf[i] == 'g')
            gcnt++;
        else if(buf[i] == 'p') pcnt++;
        else break;
        i++;
    }
    
    printf("%d\n", (gcnt - pcnt)/2);
    free(buf);
    return 0;
}
