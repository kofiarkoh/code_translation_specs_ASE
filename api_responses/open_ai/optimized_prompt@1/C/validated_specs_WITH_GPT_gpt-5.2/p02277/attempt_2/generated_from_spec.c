#include <stdio.h>

typedef struct {
    char label;
    int key;
    int pos; /* original input position */
} Record;

static void swap_record(Record *a, Record *b) {
    Record t = *a;
    *a = *b;
    *b = t;
}

static int partition(Record A[], int p, int r) {
    int x = A[r].key; /* pivot key is last element */
    int i = p - 1;
    for (int j = p; j <= r - 1; j++) {
        if (A[j].key <= x) {
            i++;
            swap_record(&A[i], &A[j]);
        }
    }
    swap_record(&A[i + 1], &A[r]);
    return i + 1;
}

static void quicksort(Record A[], int p, int r) {
    if (p < r) {
        int q = partition(A, p, r);
        quicksort(A, p, q - 1);
        quicksort(A, q + 1, r);
    }
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        return 0; /* unspecified behavior; exit quietly */
    }

    /* Automatic array as per specification assumption */
    Record A[n];

    for (int i = 0; i < n; i++) {
        /* Whitespace may appear before the label, so use " %c" */
        if (scanf(" %c %d", &A[i].label, &A[i].key) != 2) {
            return 0; /* unspecified behavior; exit quietly */
        }
        A[i].pos = i;
    }

    quicksort(A, 0, n - 1);

    int stable = 1;
    for (int i = 0; i < n; i++) {
        for (int j = i + 1; j < n; j++) {
            if (A[i].key == A[j].key && A[i].pos > A[j].pos) {
                stable = 0;
                break;
            }
        }
        if (!stable) break;
    }

    if (stable) {
        printf("Stable\n");
    } else {
        printf("Not stable\n");
    }

    for (int i = 0; i < n; i++) {
        printf("%c %d\n", A[i].label, A[i].key);
    }

    return 0;
}
