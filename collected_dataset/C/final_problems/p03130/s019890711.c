#include <stdio.h>
int main(void){
    int a[4] = {0}; int b[4] = {0};
    for(int i = 0; i < 3; i++){
        scanf("%ld %ld", &a[i], &b[i]);
    }
    int flag1 = 0; int flag2 = 0; int flag3 = 0; int flag4 = 0;
    for(int i = 0; i < 3; i++){
        if(a[i] == 1){
            flag1++;
        }else if(a[i] == 2){
            flag2++;
        }else if(a[i] == 3){
            flag3++;
        }else {
            flag4++;
        }
    }
    for(int i = 0; i < 3; i++){
        if(b[i] == 1){
            flag1++;
        }else if(b[i] == 2){
            flag2++;
        }else if(b[i] == 3){
            flag3++;
        }else {
            flag4++;
        }
    }
    int p1 = 0; int p2 = 0; int p3 = 0; int p4 = 0;
    if(flag1 > 0 && flag1 < 3){
        p1 = 1;
    }
    if(flag2 > 0 && flag2 < 3){
        p2 = 1;
    }
    if(flag3 > 0 && flag3 < 3){
        p3 = 1;
    }
    if(flag4 > 0 && flag4 < 3){
        p4 = 1;
    }

    if(p1 == 1 && p2 == 1 && p3 == 1 && p4 == 1){
        printf("YES\n");
    }else {
        printf("NO\n");
    }

    return 0;
}