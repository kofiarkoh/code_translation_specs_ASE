// same integers
#include <stdio.h>

/*int find_num(int target, int ub, int kishutsu[MAXSIZE]){
    for(int i = 0; i < ub; i++){
        if(kishutsu[i] == target) return 1;
    }
    return 0;
}*/

int* max(int a[3]){
    if(a[0] >= a[1] && a[0] >= a[2]) return &a[0];
    else if(a[1] >= a[0] && a[1] >= a[2]) return &a[1];
    else return &a[2];
}

int find_even(int ar[3]){
    int num = 0;
    for(int i = 0; i < 3; i++){
        if(ar[i] % 2 == 0) num++;
    }
    return num;
}

int main(void){
    int a[3];
    scanf("%d %d %d", &a[0], &a[1], &a[2]);
    
    int *target;
    target = max(a);
    //printf("target = %d\n", *target);
    int num_even = find_even(a);
    //printf("num_even = %d\n", num_even);
    int ans;
    if(num_even == 3 || num_even == 0){
        ans = ((*target) * 3 - a[0] - a[1] - a[2]) / 2;
    }else{
        if((*target) % 2 != num_even % 2){
            //printf("max and num_even are not the same\n");
            ans = (*target - a[0]) / 2 + (*target - a[1]) / 2 + (*target - a[2]) / 2 + 1;
        }else{
            //printf("max and num_even are the same\n");
            for(int i = 0; i < 3; i++){
                if(a[i] != (*target) && a[i] % 2 == 0){
                    a[i]++;
                    break;
                }
            }
            (*target)++;
            ans = 1 + ((*target) * 3 - a[0] - a[1] - a[2]) / 2;
        }
    }
    printf("%d\n", ans);
    return 0;
}