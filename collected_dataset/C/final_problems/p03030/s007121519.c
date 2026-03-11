#include<stdio.h>
#include<string.h>

int main(void){
    int n;
    scanf("%d", &n);
    char s[n][11];
    int p[n];
    int index[n];
    for(int i = 0; i < n; i++){
        scanf("%s", s[i]);
        scanf("%d", &p[i]);
        index[i] = i+1;
    }

    int buffer;
    char buf[11];
    for(int i = 0; i < n-1; i++){
        for(int j = i+1; j < n; j++){
            if(strcmp(s[i], s[j]) > 0){
                strcpy(buf, s[i]);
                strcpy(s[i], s[j]);
                strcpy(s[j], buf);
                buffer = p[i];
                p[i] = p[j];
                p[j] = buffer;
                buffer = index[i];
                index[i] = index[j];
                index[j] = buffer;
            }else if(strcmp(s[i], s[j]) == 0){
                if(p[i] < p[j]){
                    buffer = p[i];
                    p[i] = p[j];
                    p[j] = buffer;
                    buffer = index[i];
                    index[i] = index[j];
                    index[j] = buffer;
                }
            }
        }
    }

    for(int i = 0; i < n; i++){printf("%d\n", index[i]);}

    
    return 0;
}