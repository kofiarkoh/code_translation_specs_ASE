#include <stdio.h>
#include <string.h>
#define STRMAX 100000
void reverse(char *s){
    int len = strlen(s);
    char temp;
    for(int i = 0; i < (int)(len/2); i++){
        temp = s[i];
        s[i] = s[(len - 1) - i];
        s[len - 1 - i] = temp;
    }
}
void add(char *s, char c){
    int len = strlen(s);
    for(int i = len; i > 0; i--){
        s[i] = s[i-1];
    }
    s[0] = c;
}
void push(char *s, char c){
    int len = strlen(s);
    s[len] = c;
}
int main(void){
    char s[STRMAX+1], c;
    scanf("%s", s);
    // add(s, 't');
    // push(s, 'k');
    // printf("%s %d\n", s, strlen(s));
    long q;
    int t, f;
    scanf("%d", &q);
    for(long i = 0; i < q; i++){
        scanf("%d", &t);
        if(t == 1){
            reverse(s);
        }
        else{ // t == 2
            scanf("%d", &f);
            getchar(); // 空白を飛ばすため
            scanf("%c", &c);
            // getchar();
            if(f == 1){
                add(s, c);
            }
            else{ // f == 2
                push(s, c);
            }
        }
        // printf("%d %d %c\n", t, f, c);
    }
    printf("%s\n", s);
    return 0;
}