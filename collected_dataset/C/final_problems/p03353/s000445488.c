#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#include<limits.h>

#define rep(i,begin,end) for(int i=begin; i<end; i++)
#define lld long long int

int cmpAscStrings(const void* a, const void* b){
    return strcmp(*(char**)a, *(char**)b);
}

void initStrArray(char** p, int n, int len){
    for(int i=0; i<n; i++){
        *(p + i) = (char*)malloc(sizeof(char) * (len + 1));
    }
}

int main(){
    char s[5001];
    int k, len;
    scanf("%s", s);
    scanf("%d", &k);
    len = strlen(s);
    char* w[30000];
    initStrArray(w, 30000, 11);
    int x=0;
    rep(i, 0, len){
        char buf[10];
        rep(j, 0, 6){
            buf[j] = s[i+j];
            buf[j+1]  = '\0';
            strcpy(w[x], buf);
            x++;
            if(s[i+j] == '\0'){
                break;
            }
        }
    }
    qsort(w, x, sizeof(char*), cmpAscStrings);
    int q=1, id=0;
    while(q<k){
        id++;
        if(strcmp(w[id-1], w[id]) != 0){
            q++;
        }
    }
    printf("%s", w[id]);

    return 0;
}