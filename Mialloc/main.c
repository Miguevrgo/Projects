#include <stddef.h>
#include <sys/mman.h>

typedef struct Block {
    size_t size;
    bool free;
    struct Block* next;
} Block;

static Block* head = nullptr;

void* request_memory(size_t size) {
    size_t total_size = sizeof(Block) + size;

    void* ptr = mmap(NULL, total_size, PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);

    if (ptr == MAP_FAILED) {
        return nullptr;
    }

    Block* block = (Block*)ptr;
    block->size = size;
    block->free = false;
    block->next = nullptr;

    return block;
}

void* my_malloc(size_t size) {
    if (size == 0) {
        return nullptr;
    }

    Block* curr = head;
    Block* prev = curr;

    while (curr) {
        if (curr->free && curr->size >= size) {
            curr->free = false;
            return (void*)(curr + 1);
        }

        prev = curr;
        curr = curr->next;
    }

    Block* new_block = request_memory(size);
    if (!new_block) {
        return nullptr;
    }

    if (!head) {
        head = new_block;
    } else {
        prev->next = new_block;
    }

    return (void*)(new_block + 1);
}

void my_free(void* ptr) {
    if (!ptr) {
        return;
    }

    Block* block = (Block*)ptr - 1;
    block->free = true;
}

int main() {}
