#include <stdio.h>

typedef struct {
    char sent[10000];
    int len;
}string;

string scan(){
    string word;
    char c;
    int i = 0;
    while((c = getchar()) != '\n'){
        word.sent[i] = c;
        i++;
    }
    word.sent[i + 1] = '\0';
    word.len = i;
    return word;
}

int judge(string s, string t){
    int ans = 1, i, j;
    for(i = 0; i < t.len; i++){
        for(j = 0; j < s.len; j++){
            if(s.sent[j] == t.sent[i]){
                break;
            }else if(j + 1 == s.len){
                ans = 0;
            }
        }
        if(ans){break;}
    }
    return ans;
}

void makeCountST(string s, string t){
    long int count = 0, p_t = 0, i;
    while(1){
        for(i = 0; i < s.len; i++){
            if(p_t == t.len){
                goto RESULT;
            }else if(s.sent[i] == t.sent[p_t]){
                p_t++;
            }
            count++;
        }
    }
    RESULT:
    printf("%ld", count);
}

int main(){
    string s, t;
    s = scan();
    t = scan();
    
    if(judge(s, t)){
        makeCountST(s, t);
    }else{
        printf("-1");
    }
}