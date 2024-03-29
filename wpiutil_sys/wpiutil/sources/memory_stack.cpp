// Copyright (C) 2015-2023 Jonathan Müller and foonathan/memory contributors
// SPDX-License-Identifier: Zlib

#include "wpi/memory/memory_stack.hpp"

#include "wpi/memory/debugging.hpp"

using namespace wpi::memory;

void detail::memory_stack_leak_handler::operator()(std::ptrdiff_t amount)
{
    get_leak_handler()({WPI_MEMORY_LOG_PREFIX "::memory_stack", this}, amount);
}

#if WPI_MEMORY_EXTERN_TEMPLATE
template class wpi::memory::memory_stack<>;
template class wpi::memory::memory_stack_raii_unwind<memory_stack<>>;
template class wpi::memory::allocator_traits<memory_stack<>>;
template class wpi::memory::composable_allocator_traits<memory_stack<>>;
#endif
