#include <stdio.h>
#include <stdlib.h>

int compare_int(const void *a, const void *b){
    return *(int*)a - *(int*)b;
}
int minimum_vote(){
    int vote[10000];
    int index = 0;
    int sum = 0;
    char c = getchar();
    while(1){
        if(c == '['){
            vote[index++] = minimum_vote();
            c = getchar();
        }
        else if(c == ']'){
            qsort(vote, index, sizeof(int), compare_int);
            int i;
            for(i = 0; i < (index + 1) / 2; i++){
                sum += vote[i];
            }
            return sum;
        }
        else{
            int t = c - '0';
            c = getchar();
            while(c >= '0' && c <= '9'){
                t = t * 10 + (c - '0');
                c = getchar();
            }
            vote[index++] = (t + 1) / 2;
        }
    }
}
int main(void){
    int n, i;
    scanf("%d", &n);
    for(i = 0; i < n; i++){
        while(getchar() != '[');
        printf("%d\n", minimum_vote());
    }
    return 0;

}