#include <stdio.h>
#define LEN 10
#define N 100000
typedef char key;

typedef struct node{
    char s[LEN+1];
    struct node *next;
}Node;

void sort_str(char s[]){
    int i,j,tmp;
    for(i = 1; i < LEN; i++){
        tmp = s[i];
        if(s[i - 1] > tmp){
            j = i;
            do{
                s[j] = s[j - 1];
                j--;
            }while(j > 0 && s[j - 1] > tmp);
            s[j] = tmp;
        }
    }
}

void sort_node(Node s[], Node work[], int n){
    Node top[26];
    Node *current;
    int i,j,k;
    int c;
    
    for(i = LEN-1; i >= 0; i--){
        for(j = 0; j < 26; j++){
            top[j].next = NULL;
        }
        
        for(j = 0; j < n; j++){
            s[j].next = NULL;
            work[j] = s[j];
        }
        
        for(j = 0; j < n; j++){
            c = work[j].s[i] - 'a';
            
            if(top[c].next == NULL){
                top[c].next = &(work[j]);
            }else{
                current = &top[c];
                while(current->next != NULL){
                    current = current->next;
                }
                current->next = &(work[j]);
            }
        }
        
        k = 0;
        current = top[k].next;
        top[k].next = NULL;
        for(j = 0; j < n;){
            if(current == NULL){
                //printf("NULL:%p ", current);
                while(k < 25 && top[k].next == NULL){
                    k++;
                }
                current = top[k].next;
                top[k].next = NULL;
            }else{
                //printf("Not NULL:%p ", current);
                s[j] = *current;
                current = current->next;
                //printf("s[%d]:%s ", j, s[j].s);
                //printf("current:%p k:%d\n", current, k);
                j++;
            }
            //printf("k:%d ", k);
        }
        /*
        for(j = 0; j < n; j++){
            printf("%s\n", s[j].s);
        }
        putchar('\n');
        */
    }
    
}

int comp(char a[], char b[]){
    int i;
    for(i = 0; i < LEN+1; i++){
        if(a[i] != b[i]){
            break;
        }
    }
    
    if(i == LEN+1){
        return 1;
    }else{
        return 0;
    }
}

int main(void){
    int n;
    static Node str[N] = {};
    static Node work[N] = {};
    int i,start;
    unsigned long long int count = 0;
    
    scanf("%d", &n);
    
    for(i = 0; i < n; i++){
        scanf("%s", str[i].s);
    }
    
    for(i = 0; i < n; i++){
        sort_str(str[i].s);
        //printf("%s\n", str[i].s);
    }
    //puts("1st");
    
    sort_node(str, work, n);
    for(i = 0; i < n; i++){
        //printf("%s\n", str[i].s);
    }
    
    
    i = 0;
    start = 0;
    while(i < n){
        if(comp(str[i].s, str[i+1].s)){
            i++;
        }else{
            count += (i - start) * (i - start + 1) / 2;
            start = i+1;
            i++;
        }
    }
    printf("%llu", count);
    
    return 0;
}