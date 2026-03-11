#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <ctype.h>

#define SIZE 100005
#define PI acosl(-1) //3.14159265358979323846264338327950L 
#define rep(i, N) for (i = 0; i < N; i++) //制御変数iを用いてN回転
#define array(N, t) (t*)calloc(N, sizeof(t)) //t型N要素の1次元配列を動的確保後0クリア(freeを忘れずに)
#define zero(a); {int iter; rep(iter, sizeof(a)/sizeof((a)[0])) (a)[iter]=0;} //1次元配列aのゼロ初期化?
#define Sort(a, t) qsort(a, sizeof(a)/sizeof((a)[0]), sizeof(t), comp) //compを使って1次元配列をソート
#define max(a, b) (a > b) ? (a) : (b)
#define min(a, b) (a < b) ? (a) : (b)

typedef long long ll; typedef long double ld;
typedef struct node { int data; struct node *next; } Node;

Node *list[SIZE];
int queue[SIZE]; int q_head = 0, q_tail = 0;

void print_log(); //とりあえず文字を出力してどこまで実行できているか確認(for debug)
void printa_int(int *a, int size); //int型の1次元配列aを出力(for debug)
void printl_int(int size); //リンクリストを出力(for debug)
int comp(const void *a, const void *b); //昇順並び替えに使う比較関数
Node* add_node(int i, int data); void free_list(Node* head);
int queue_empty(); int queue_full();
void enqueue(int data); int dequeue();
ll* factorize(ll N); //Nを素因数分解、得られる配列は素因数,指数,素因数,…となっている(添字はintで指定する必要あり?)
void check_factorization(ll *fact); //素因数分解を表示(for debug)

int main() {
    int i, K;
    ll num = 0;
    scanf("%d", &K);
    int *count = array(K, int);
    for (i = 1;; i++) {
        num = num * 10 + 7;
        if (num % K == 0) {
            printf("%d\n", i);
            return 0;
        }
        num %= K;
        count[num]++;
        if (count[num] > 1) break;
    }
    printf("-1\n");
    free(count);
    return 0;
}

void print_log() {
    printf("=== print log ===\n");
}
void printa_int(int *a, int size) {
    int i;
    printf("[");
    for (i = 0; i < size; i++)
        printf("%d, ", a[i]);
    printf("]\n");
}
void printl_int(int size) {
    int i;
    Node *t;
    rep(i, size) {
        printf("[%d] ", i);
        for (t = list[i]; t; t = t->next) printf("%d, ", t->data);
        printf("\n");
    }
}
int comp(const void *a, const void *b) {
    return *(int*)a - *(int*)b;
}
Node* add_node(int i, int data) {
    if (!list[i]) {
        list[i] = (Node*)malloc(sizeof(Node*));
        list[i]->data = data;
    } else {
        Node *t;
        t = list[i];
        list[i] = (Node*)malloc(sizeof(Node*));
        list[i]->data = data;
        list[i]->next = t;
    }
    return list[i];
}
void free_list(Node* head) {
    Node *prev = NULL, *t = head;
    while (t) {
        prev = t;
        t = t->next;
        free(prev);
    }
}
int queue_empty() {
    return q_head == q_tail;
}
int queue_full() {
    return (q_head - 1) % SIZE == q_tail % SIZE;
}
void enqueue(int data) {
    if (queue_full()) {
        printf("full!\n");
        return;
    }
    queue[q_tail++] = data;
    q_tail %= SIZE;
}
int dequeue() {
    if (queue_empty()) {
        printf("empty!\n");
        return -444444;
    }
    int data = queue[q_head++];
    q_head %= SIZE;
    return data;
}
ll* factorize(ll N) {
    ll *res = array(((sqrt(N) + 1) * 2), ll);
    ll i, x = N;
    int count = 0;
    for (i = 2; i * i <= N; i++) {
        ll e = 0;
        while (x % i == 0) {
            x /= i;
            e++;
        }
        if (e > 0) {
            res[count * 2] = i;
            res[count * 2 + 1] = e;
            count++;
        }
    }
    if (x > 1) {
        res[count * 2] = x;
        res[count * 2 + 1] = 1;
    }
    return res;
}
void check_factorization(ll *fact) {
    int i = 1;
    printf("%lld^%lld", fact[0], fact[1]);
    while (fact[i * 2] > 0) {
        printf(" * %lld^%lld", fact[i * 2], fact[i * 2 + 1]);
        i++;
    }
    printf("\n");
}