#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define NUM 1000000

typedef struct{
    int *v;
    int *prev;
    int *next;
    
    int first;
    int last;
    int free_first;
    
    int cnt;
} deque;

void init(deque *d){
	
	d->v = malloc(sizeof(int) * NUM);
	d->prev = malloc(sizeof(int) * NUM);
	d->next = malloc(sizeof(int) * NUM);

    d->first = 0;
    d->last = 0;
    d->free_first = 0;
    d->cnt = 0;
    
    d->v[0] = 0;
    d->prev[0] = -1;
    d->next[0] = 1;
    
    int i;
    
    for(i = 1; i < NUM - 1; i++){
        d->v[i] = 0;
        d->prev[i] = i - 1;
        d->next[i] = i + 1;
    }
    
    d->v[NUM - 1] = 0;
    d->prev[NUM - 1] = NUM - 2;
    d->next[NUM - 1] = -1;
}

void insert(deque *d, int n){
    
    int new_addr = d->free_first;
    int old_first;
    
    if (d->cnt == 0){
      old_first = -1;
    }else old_first = d->first;
    
    d->free_first = d->next[new_addr];
    d->prev[d->free_first] = -1;
    
    d->prev[new_addr] = -1;
    d->next[new_addr] = old_first;
    d->v[new_addr] = n;
    
    (d->cnt)++;
    
    d->prev[old_first] = new_addr;
    
    d->first = new_addr;
    if (d->cnt == 1) d->last = new_addr;
    
}

void push_free(deque *d, int addr){
    
    int old_free_first = d->free_first;
    
    d->free_first = addr;
    d->prev[addr] = -1;
    d->next[addr] = old_free_first;
    
    d->prev[old_free_first] = addr;
}

void delete(deque *d, int n){
    
    int addr = d->first;
    
    if (d->cnt == 1) {
    
        d->first = -1;
        d->last = -1;
        
        push_free(d, addr);
        (d->cnt)--;
    
    }else while (addr != -1){
        if (d->v[addr] == n){
        
            if (addr == d->first) {
                d->prev[d->next[addr]] = -1;
                d->first = d->next[addr];
            }else if (addr == d->last) {
                d->next[d->prev[addr]] = -1;
                d->last = d->prev[addr];
            }else {
                d->prev[d->next[addr]] = d->prev[addr];
                d->next[d->prev[addr]] = d->next[addr];
            }
            
            push_free(d, addr);
            (d->cnt)--;
        
            break;
        }
        addr = d->next[addr];
    }
}

void delete_first(deque *d){

    int addr = d->first;

    if(d->cnt != 1){
        d->prev[d->next[addr]] = -1;
        d->first = d->next[addr];
        
    }else{
        d->first = -1;
        d->last = -1;
    }
    
    push_free(d, addr);
    (d->cnt)--;
    
}

void delete_last(deque *d){

    int addr = d->last;

    if(d->cnt != 1){
        d->next[d->prev[addr]] = -1;
        d->last = d->prev[addr];
        
    }else{
        d->first = -1;
        d->last = -1;
    }
    
    push_free(d, addr);
    (d->cnt)--;
    
}

void print_deque(deque *d){
    
    int addr = d->first;
    
    while(addr != -1){
        if (addr == d->last) {
            printf("%d", d->v[addr]);
        }else printf("%d ",d->v[addr]);
        addr = d->next[addr];
    }
    
    printf("\n");
}

/*
void debug_print(deque *d){

    int i;
    
    printf("\naddr ");
    for (i = 0; i < NUM; i++) {
        printf("%2d ",i);
    }
    
    printf("\nprev ");
    for (i = 0; i < NUM; i++) {
        printf("%2d ",d->prev[i]);
    }
    
    printf("\nnext ");
    for (i = 0; i < NUM; i++) {
        printf("%2d ",d->next[i]);
    }
    
    printf("\n   v ");
    for (i = 0; i < NUM; i++) {
        printf("%2d ",d->v[i]);
    }
    printf("\n cnt %2d", d->cnt);
    printf("\nfirs %2d", d->first);
    printf("\nlast %2d", d->last);
    
}
*/

int main(void) {

  deque de;
  int n, x, i;
  char s[12];
  
  init(&de);
  
  scanf("%d", &n);
  
  for(i = 0; i < n; i++){
    scanf("%s %d", s, &x);
    
    if(strcmp(s, "insert") == 0){
        insert(&de, x);
    }else if (strcmp(s, "delete") == 0) {
        delete(&de, x);
    }else if (strcmp(s, "deleteFirst") == 0) {
        delete_first(&de);
    }else if (strcmp(s, "deleteLast") == 0) {
        delete_last(&de);
    }
    
    /*
    debug_print(&de);
    printf("\n");
    print_deque(&de);
    printf("\n");
    */
  }
  
  print_deque(&de);

  return 0;
}