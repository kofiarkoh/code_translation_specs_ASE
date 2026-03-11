#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char command[7];
char dictionary[13];

typedef struct ha_tag{

   char m[13];
   struct ha_tag* next;
}hash;

hash array[10000];


int calc(char* word){
int w = 0, i = 0;

   while(word[i] != '\0'){
     w += word[i] * 31;
    i++;
   }
   if(w == 0){
    return 0;
   }

  return (w % 10000) + 1;
}

void insert(char* word){
hash* h;

  if(array[calc(word)].next == 0){

    strcpy(array[calc(word)].m, word);
    array[calc(word)].next = malloc(sizeof(hash));
    array[calc(word)].next->next = 0;
    strcpy(array[calc(word)].next->m, "\x00");
  }
  else if(strcmp(array[calc(word)].m, word) == 0){
    return;
  }
  else{
   h = array[calc(word)].next;
   while(h->next != 0){
    h = h->next;

   }
  strcpy(h->m, word);
  h->next = malloc(sizeof(hash));
  h->next->next = 0;
  h->next->m[0] = '\0';
  }
}

int find(char* word){
hash* h;
h  = &array[calc(word)];
  if(h->m[0] == '\0')
    return 0;

  while(strcmp(h->m, word) != 0){

    h = h->next;
    if(h->next == 0){
     return 0;
    }
  }

  return 1;
}



int main(){
int n, i = 0, f = 0;
int flag[1000];
scanf("%d", &n);

  for(i; i < n; i++){
    scanf("%s", command);
    scanf("%s", dictionary);
    if(strcmp(command, "insert") == 0)
     insert(dictionary);

   if(strcmp(command, "find") == 0){

    flag[f] = find(dictionary);

    f += 1;

   }

  }
 for(i = 0; i < f; i++)
  printf("%s", flag[i] == 1 ? "yes\n":"no\n");

 return 0;
}