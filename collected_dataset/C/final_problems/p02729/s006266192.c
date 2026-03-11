#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char* argv[]){
        char buf[64]={0};
        char* p=NULL;
        int odd_num = 0;
        int evn_num = 0;
        int ans1 = 0;
        int ans2 = 0;

        if(fgets(buf, sizeof(buf)-1, stdin)==NULL){
                return 1;
        }
        p = strtok(buf, " ");
        evn_num = atoi(p);
        if(evn_num<0 || evn_num>100){
                return 1;
        }
        p = strtok(NULL, " ");
        odd_num = atoi(p);
        if(odd_num<0 || odd_num>100){
                return 2;
        }
        ans1 = evn_num*(evn_num-1);
        ans1 = ans1/2;
        ans2 = odd_num*(odd_num-1);
        ans2 = ans2/2;
        printf("%d\n", ans1+ans2);
        return 0;
}